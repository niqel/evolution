use evo_shell_engine::{
    Advance, Filter, Index, Iter, Select, Take, ToArgs, ToValue, ToValues, argument_expander,
    filterer, indexer, iteration_advancer, iterator, selector, taker, value_converter,
    values_converter,
};

use crate::definitions::domain::entities::shell::Shell;
use crate::definitions::domain::value_objects::pipeline::{
    Pipeline, PipelineOperation, PipelineOperationKind,
};
use crate::definitions::domain::value_objects::pipeline_value::{
    PipelineItems, PipelineValue, PipelineValueKind,
};
use crate::definitions::use_cases::execute_pipeline::PipelineExecutionError;

pub fn execute(shell: &Shell, pipeline: Pipeline) -> Result<PipelineValue, PipelineExecutionError> {
    let iter: Iter = iterator::iter;
    let advance: Advance = iteration_advancer::advance;
    let filter: Filter = filterer::filter;
    let index: Index = indexer::index;
    let take: Take = taker::take;
    let select: Select = selector::select;
    let to_value_case: ToValue = value_converter::convert;
    let to_values_case: ToValues = values_converter::convert;
    let to_args_case: ToArgs = argument_expander::expand;

    execute_with(
        shell,
        pipeline,
        iter,
        advance,
        filter,
        index,
        take,
        select,
        to_value_case,
        to_values_case,
        to_args_case,
    )
}

pub(crate) fn execute_with(
    shell: &Shell,
    pipeline: Pipeline,
    iter: Iter,
    advance: Advance,
    filter: Filter,
    index: Index,
    take: Take,
    select: Select,
    to_value_case: ToValue,
    to_values_case: ToValues,
    to_args_case: ToArgs,
) -> Result<PipelineValue, PipelineExecutionError> {
    let mut operations = pipeline.into_operations();

    if operations.is_empty() {
        return Err(PipelineExecutionError::EmptyPipeline);
    }

    let first = operations.remove(0);
    let mut state = execute_initial_operation(shell, first, iter, advance)?;

    for operation in operations {
        state = execute_next_operation(
            state,
            operation,
            filter,
            index,
            take,
            select,
            to_value_case,
            to_values_case,
            to_args_case,
        )?;
    }

    Ok(state)
}

fn execute_initial_operation(
    shell: &Shell,
    operation: PipelineOperation,
    iter: Iter,
    advance: Advance,
) -> Result<PipelineValue, PipelineExecutionError> {
    match operation {
        PipelineOperation::Iter => {
            materialize_iter(shell, iter, advance).map(PipelineValue::StructuredItems)
        }
        _ => Err(PipelineExecutionError::InvalidInitialOperation {
            operation: operation.kind(),
        }),
    }
}

fn execute_next_operation(
    state: PipelineValue,
    operation: PipelineOperation,
    filter: Filter,
    index: Index,
    take: Take,
    select: Select,
    to_value_case: ToValue,
    to_values_case: ToValues,
    to_args_case: ToArgs,
) -> Result<PipelineValue, PipelineExecutionError> {
    match operation {
        PipelineOperation::Filter(expression) => match state {
            PipelineValue::StructuredItems(items) => {
                let filtered = filter(items.structured_items(), &expression)
                    .map_err(PipelineExecutionError::from)?;
                let selection = items.selection_from(filtered);
                let items = items.into_items();
                Ok(PipelineValue::StructuredItems(
                    PipelineItems::with_selection(items, selection),
                ))
            }
            state => Err(invalid_transition(
                PipelineOperationKind::Filter,
                state.kind(),
            )),
        },
        PipelineOperation::Index(target_index) => match state {
            PipelineValue::StructuredItems(items) => {
                let indexed = index(items.structured_items(), target_index)
                    .map_err(PipelineExecutionError::from)?;
                let selection = items.selection_from(indexed);
                let items = items.into_items();
                Ok(PipelineValue::StructuredItems(
                    PipelineItems::with_selection(items, selection),
                ))
            }
            state => Err(invalid_transition(
                PipelineOperationKind::Index,
                state.kind(),
            )),
        },
        PipelineOperation::Take(count) => match state {
            PipelineValue::StructuredItems(items) => {
                let taken = take(items.structured_items(), count);
                let selection = items.selection_from(taken);
                let items = items.into_items();
                Ok(PipelineValue::StructuredItems(
                    PipelineItems::with_selection(items, selection),
                ))
            }
            state => Err(invalid_transition(
                PipelineOperationKind::Take,
                state.kind(),
            )),
        },
        PipelineOperation::Select(properties) => match state {
            PipelineValue::StructuredItems(items) => {
                let projection = select(items.structured_items(), &properties)
                    .map_err(PipelineExecutionError::from)?;
                Ok(PipelineValue::StructuredProjection(projection))
            }
            state => Err(invalid_transition(
                PipelineOperationKind::Select,
                state.kind(),
            )),
        },
        PipelineOperation::ToValue => match state {
            PipelineValue::StructuredProjection(projection) => {
                let value = to_value_case(projection).map_err(PipelineExecutionError::from)?;
                Ok(PipelineValue::Value(value))
            }
            state => Err(invalid_transition(
                PipelineOperationKind::ToValue,
                state.kind(),
            )),
        },
        PipelineOperation::ToValues => match state {
            PipelineValue::StructuredProjection(projection) => {
                let values = to_values_case(projection).map_err(PipelineExecutionError::from)?;
                Ok(PipelineValue::Values(values))
            }
            state => Err(invalid_transition(
                PipelineOperationKind::ToValues,
                state.kind(),
            )),
        },
        PipelineOperation::ToArgs => match state {
            PipelineValue::StructuredProjection(projection) => {
                let arguments = to_args_case(projection).map_err(PipelineExecutionError::from)?;
                Ok(PipelineValue::Arguments(arguments))
            }
            state => Err(invalid_transition(
                PipelineOperationKind::ToArgs,
                state.kind(),
            )),
        },
        PipelineOperation::Iter => Err(invalid_transition(
            PipelineOperationKind::Iter,
            state.kind(),
        )),
    }
}

fn invalid_transition(
    operation: PipelineOperationKind,
    state: PipelineValueKind,
) -> PipelineExecutionError {
    PipelineExecutionError::InvalidTransition { operation, state }
}

fn materialize_iter(
    shell: &Shell,
    iter: Iter,
    advance: Advance,
) -> Result<PipelineItems, PipelineExecutionError> {
    let mut iteration = iter(shell.filesystem_scope()).map_err(PipelineExecutionError::from)?;
    let mut items = Vec::new();

    while let Some(item) = advance(&mut iteration).map_err(PipelineExecutionError::from)? {
        items.push(item);
    }

    Ok(PipelineItems::new(items))
}

#[cfg(test)]
mod tests {
    use super::*;
    use evo_shell_engine::{
        FilterComparison, FilterExpression, FilterOperand, FilterOperator, FilterProperty,
        FilterValue, ProjectedValue, SelectProperty, StructuredItems, StructuredProjection,
        argument_expander, filterer, iteration_advancer, iterator, scope_setter, selector, taker,
        value_converter, values_converter,
    };
    use std::ffi::OsString;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_directory(prefix: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after UNIX_EPOCH")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "evo_shell_pipeline_{prefix}_{}_{}",
            std::process::id(),
            unique
        ));
        fs::create_dir(&path).expect("temporary test directory should be created");
        path
    }

    fn shell_from_path(path: &PathBuf) -> Shell {
        Shell::new(scope_setter::set(path.as_path()).unwrap())
    }

    fn one_file_directory(name: &str) -> PathBuf {
        let path = temp_directory(name);
        fs::write(path.join("only.txt"), b"content").unwrap();
        path
    }

    fn two_file_directory(name: &str) -> PathBuf {
        let path = temp_directory(name);
        fs::write(path.join("keep.txt"), b"content").unwrap();
        fs::write(path.join("drop.txt"), b"content").unwrap();
        path
    }

    #[test]
    fn pipeline_value_kind_matches_variant() {
        let items = PipelineItems::new(vec![]);
        assert_eq!(
            PipelineValue::StructuredItems(items).kind(),
            PipelineValueKind::StructuredItems
        );
    }

    #[test]
    fn execute_pipeline_matches_use_case_function_pointer() {
        let execute_pipeline: crate::definitions::use_cases::execute_pipeline::ExecutePipeline =
            execute;
        let _ = execute_pipeline;
    }

    #[test]
    fn vertical_pipeline_iter_take_select_to_value_returns_typed_value() {
        let directory = one_file_directory("vertical_value");
        let shell = shell_from_path(&directory);
        let pipeline = Pipeline::new(vec![
            PipelineOperation::Iter,
            PipelineOperation::Take(1),
            PipelineOperation::Select(vec![SelectProperty::Name]),
            PipelineOperation::ToValue,
        ]);

        let result = execute(&shell, pipeline).unwrap();

        match result {
            PipelineValue::Value(value) => {
                assert_eq!(value, ProjectedValue::Name(OsString::from("only.txt")));
            }
            other => panic!("unexpected result: {other:?}"),
        }
    }

    #[test]
    fn vertical_pipeline_iter_select_to_values_returns_values() {
        let directory = one_file_directory("vertical_values");
        let shell = shell_from_path(&directory);
        let pipeline = Pipeline::new(vec![
            PipelineOperation::Iter,
            PipelineOperation::Select(vec![SelectProperty::Name]),
            PipelineOperation::ToValues,
        ]);

        let result = execute(&shell, pipeline).unwrap();

        match result {
            PipelineValue::Values(values) => {
                assert_eq!(values.len(), 1);
                assert_eq!(
                    values.items(),
                    &[ProjectedValue::Name(OsString::from("only.txt"))]
                );
            }
            other => panic!("unexpected result: {other:?}"),
        }
    }

    #[test]
    fn vertical_pipeline_iter_select_to_args_returns_arguments() {
        let directory = one_file_directory("vertical_args");
        let shell = shell_from_path(&directory);
        let pipeline = Pipeline::new(vec![
            PipelineOperation::Iter,
            PipelineOperation::Select(vec![SelectProperty::Name]),
            PipelineOperation::ToArgs,
        ]);

        let result = execute(&shell, pipeline).unwrap();

        match result {
            PipelineValue::Arguments(arguments) => {
                assert_eq!(arguments.len(), 1);
                assert_eq!(
                    arguments.items(),
                    &[ProjectedValue::Name(OsString::from("only.txt"))]
                );
            }
            other => panic!("unexpected result: {other:?}"),
        }
    }

    #[test]
    fn pipeline_filter_delegates_to_engine() {
        let directory = two_file_directory("filter");
        let shell = shell_from_path(&directory);
        let pipeline = Pipeline::new(vec![
            PipelineOperation::Iter,
            PipelineOperation::Filter(FilterExpression::comparison(FilterComparison::new(
                FilterProperty::Name,
                FilterOperator::Equals,
                FilterOperand::single(FilterValue::name("keep.txt")),
            ))),
            PipelineOperation::Select(vec![SelectProperty::Name]),
            PipelineOperation::ToValues,
        ]);

        let result = execute(&shell, pipeline).unwrap();

        match result {
            PipelineValue::Values(values) => {
                assert_eq!(values.len(), 1);
                assert_eq!(
                    values.items(),
                    &[ProjectedValue::Name(OsString::from("keep.txt"))]
                );
            }
            other => panic!("unexpected result: {other:?}"),
        }
    }

    #[test]
    fn pipeline_index_delegates_to_engine() {
        let directory = one_file_directory("index");
        let shell = shell_from_path(&directory);
        let pipeline = Pipeline::new(vec![
            PipelineOperation::Iter,
            PipelineOperation::Index(0),
            PipelineOperation::Select(vec![SelectProperty::Name]),
            PipelineOperation::ToValue,
        ]);

        let result = execute(&shell, pipeline).unwrap();

        match result {
            PipelineValue::Value(value) => {
                assert_eq!(value, ProjectedValue::Name(OsString::from("only.txt")));
            }
            other => panic!("unexpected result: {other:?}"),
        }
    }

    #[test]
    fn pipeline_rejects_invalid_transition_from_iter_to_to_value() {
        let directory = one_file_directory("invalid_transition");
        let shell = shell_from_path(&directory);
        let pipeline = Pipeline::new(vec![PipelineOperation::Iter, PipelineOperation::ToValue]);

        let result = execute(&shell, pipeline);

        assert!(matches!(
            result,
            Err(PipelineExecutionError::InvalidTransition {
                operation: PipelineOperationKind::ToValue,
                state: PipelineValueKind::StructuredItems
            })
        ));
    }

    #[test]
    fn pipeline_rejects_iter_not_initial() {
        let directory = one_file_directory("iter_not_initial");
        let shell = shell_from_path(&directory);
        let pipeline = Pipeline::new(vec![
            PipelineOperation::Iter,
            PipelineOperation::Take(1),
            PipelineOperation::Iter,
        ]);

        let result = execute(&shell, pipeline);

        assert!(matches!(
            result,
            Err(PipelineExecutionError::InvalidTransition {
                operation: PipelineOperationKind::Iter,
                state: PipelineValueKind::StructuredItems
            })
        ));
    }

    #[test]
    fn pipeline_rejects_empty_pipeline() {
        let directory = one_file_directory("empty_pipeline");
        let shell = shell_from_path(&directory);
        let pipeline = Pipeline::new(Vec::new());

        let result = execute(&shell, pipeline);

        assert!(matches!(result, Err(PipelineExecutionError::EmptyPipeline)));
    }

    #[test]
    fn pipeline_fail_fast_stops_after_failing_index() {
        static INDEX_CALLS: AtomicUsize = AtomicUsize::new(0);
        static SELECT_CALLS: AtomicUsize = AtomicUsize::new(0);
        static VALUE_CALLS: AtomicUsize = AtomicUsize::new(0);

        fn failing_index(
            _items: StructuredItems<'_>,
            index: usize,
        ) -> Result<StructuredItems<'_>, evo_shell_engine::IndexError> {
            INDEX_CALLS.fetch_add(1, Ordering::SeqCst);
            Err(evo_shell_engine::IndexError::not_found(index))
        }

        fn tracking_select(
            items: StructuredItems<'_>,
            properties: &[SelectProperty],
        ) -> Result<StructuredProjection, evo_shell_engine::SelectError> {
            SELECT_CALLS.fetch_add(1, Ordering::SeqCst);
            selector::select(items, properties)
        }

        fn tracking_to_value(
            projection: StructuredProjection,
        ) -> Result<ProjectedValue, evo_shell_engine::ToValueError> {
            VALUE_CALLS.fetch_add(1, Ordering::SeqCst);
            value_converter::convert(projection)
        }

        let directory = one_file_directory("fail_fast");
        let shell = shell_from_path(&directory);
        let pipeline = Pipeline::new(vec![
            PipelineOperation::Iter,
            PipelineOperation::Index(999),
            PipelineOperation::Select(vec![SelectProperty::Name]),
            PipelineOperation::ToValue,
        ]);

        let result = execute_with(
            &shell,
            pipeline,
            iterator::iter,
            iteration_advancer::advance,
            filterer::filter,
            failing_index,
            taker::take,
            tracking_select,
            tracking_to_value,
            values_converter::convert,
            argument_expander::expand,
        );

        assert!(matches!(result, Err(PipelineExecutionError::Index(_))));
        assert_eq!(INDEX_CALLS.load(Ordering::SeqCst), 1);
        assert_eq!(SELECT_CALLS.load(Ordering::SeqCst), 0);
        assert_eq!(VALUE_CALLS.load(Ordering::SeqCst), 0);
    }
}
