use evo_shell_engine::{FilterExpression, SelectProperty};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pipeline {
    operations: Vec<PipelineOperation>,
}

impl Pipeline {
    pub fn new(operations: Vec<PipelineOperation>) -> Self {
        Self { operations }
    }

    pub fn operations(&self) -> &[PipelineOperation] {
        &self.operations
    }

    pub fn into_operations(self) -> Vec<PipelineOperation> {
        self.operations
    }

    pub fn is_empty(&self) -> bool {
        self.operations.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineOperation {
    Iter,
    Filter(FilterExpression),
    Index(usize),
    Take(usize),
    Select(Vec<SelectProperty>),
    ToValue,
    ToValues,
    ToArgs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineOperationKind {
    Iter,
    Filter,
    Index,
    Take,
    Select,
    ToValue,
    ToValues,
    ToArgs,
}

impl PipelineOperation {
    pub fn kind(&self) -> PipelineOperationKind {
        match self {
            Self::Iter => PipelineOperationKind::Iter,
            Self::Filter(_) => PipelineOperationKind::Filter,
            Self::Index(_) => PipelineOperationKind::Index,
            Self::Take(_) => PipelineOperationKind::Take,
            Self::Select(_) => PipelineOperationKind::Select,
            Self::ToValue => PipelineOperationKind::ToValue,
            Self::ToValues => PipelineOperationKind::ToValues,
            Self::ToArgs => PipelineOperationKind::ToArgs,
        }
    }

    pub fn index(&self) -> Option<usize> {
        match self {
            Self::Index(index) => Some(*index),
            _ => None,
        }
    }

    pub fn take_count(&self) -> Option<usize> {
        match self {
            Self::Take(count) => Some(*count),
            _ => None,
        }
    }

    pub fn filter_expression(&self) -> Option<&FilterExpression> {
        match self {
            Self::Filter(expression) => Some(expression),
            _ => None,
        }
    }

    pub fn select_properties(&self) -> Option<&[SelectProperty]> {
        match self {
            Self::Select(properties) => Some(properties),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use evo_shell_engine::{
        FilterComparison, FilterOperand, FilterOperator, FilterProperty, FilterValue,
    };

    #[test]
    fn pipeline_preserves_operation_order() {
        let pipeline = Pipeline::new(vec![
            PipelineOperation::Iter,
            PipelineOperation::Take(1),
            PipelineOperation::Select(vec![SelectProperty::Name]),
            PipelineOperation::ToValue,
        ]);

        assert_eq!(pipeline.operations()[0], PipelineOperation::Iter);
        assert_eq!(pipeline.operations()[1], PipelineOperation::Take(1));
        assert_eq!(
            pipeline.operations()[2],
            PipelineOperation::Select(vec![SelectProperty::Name])
        );
        assert_eq!(pipeline.operations()[3], PipelineOperation::ToValue);
    }

    #[test]
    fn operation_accessors_preserve_payloads() {
        let filter =
            PipelineOperation::Filter(FilterExpression::comparison(FilterComparison::new(
                FilterProperty::Name,
                FilterOperator::Equals,
                FilterOperand::single(FilterValue::name("README.md")),
            )));

        assert_eq!(PipelineOperation::Index(7).index(), Some(7));
        assert_eq!(PipelineOperation::Take(2).take_count(), Some(2));
        assert_eq!(
            filter.filter_expression(),
            Some(&FilterExpression::comparison(FilterComparison::new(
                FilterProperty::Name,
                FilterOperator::Equals,
                FilterOperand::single(FilterValue::name("README.md")),
            )))
        );
        assert_eq!(
            PipelineOperation::Select(vec![SelectProperty::Name]).select_properties(),
            Some(&[SelectProperty::Name][..])
        );
    }

    #[test]
    fn pipeline_operation_kind_matches_variant() {
        assert_eq!(PipelineOperation::Iter.kind(), PipelineOperationKind::Iter);
        assert_eq!(
            PipelineOperation::ToArgs.kind(),
            PipelineOperationKind::ToArgs
        );
    }

    #[test]
    fn pipeline_supports_move_out_operations() {
        let pipeline = Pipeline::new(vec![PipelineOperation::Select(vec![SelectProperty::Name])]);
        let operations = pipeline.into_operations();

        assert_eq!(
            operations,
            vec![PipelineOperation::Select(vec![SelectProperty::Name])]
        );
    }
}
