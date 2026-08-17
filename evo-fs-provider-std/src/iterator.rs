use evo_shell::definitions::contracts::iterate as iterate_contract;
use evo_shell::definitions::requesters::construction_requester;
use evo_shell::definitions::structs::borrowed::between_condition::BetweenCondition;
use evo_shell::definitions::structs::borrowed::condition::Condition;
use evo_shell::definitions::structs::borrowed::condition_expression::ConditionExpression;
use evo_shell::definitions::structs::borrowed::construction::Construction;
use evo_shell::definitions::structs::borrowed::field::Field;
use evo_shell::definitions::structs::borrowed::in_condition::InCondition;
use evo_shell::definitions::structs::borrowed::iteration::Iteration;
use evo_shell::definitions::structs::borrowed::iteration_operation::IterationOperation;
use evo_shell::definitions::structs::borrowed::record::Record;
use evo_shell::definitions::structs::borrowed::value::Value;
use evo_shell::definitions::structs::owned::condition_operator::ConditionOperator;
use evo_shell::definitions::structs::owned::flow::Flow;

fn find_field<'a>(record: &'a Record<'_>, field_name: &str) -> Option<&'a Field<'a>> {
    record.fields.iter().find(|f| f.name == field_name)
}

fn matches_condition<'iteration>(
    condition: &Condition<'iteration>,
    record: &Record<'_>,
) -> Result<bool, iterate_contract::Error<'iteration>> {
    let field = find_field(record, condition.field)
        .ok_or(iterate_contract::Error::FieldNotFound(condition.field))?;

    match condition.operator {
        ConditionOperator::Equal => match (field.value, condition.value) {
            (Value::Text(actual), Value::Text(expected)) => Ok(actual == expected),
            (Value::Unsigned(actual), Value::Unsigned(expected)) => Ok(actual == expected),
            (Value::Signed(actual), Value::Signed(expected)) => Ok(actual == expected),
            (Value::Boolean(actual), Value::Boolean(expected)) => Ok(actual == expected),
            _ => Err(iterate_contract::Error::ComparisonTypeMismatch(
                condition.field,
            )),
        },
        ConditionOperator::GreaterThan => match (field.value, condition.value) {
            (Value::Unsigned(actual), Value::Unsigned(expected)) => Ok(actual > expected),
            (Value::Signed(actual), Value::Signed(expected)) => Ok(actual > expected),
            _ => Err(iterate_contract::Error::ComparisonTypeMismatch(
                condition.field,
            )),
        },
        ConditionOperator::LessThan => match (field.value, condition.value) {
            (Value::Unsigned(actual), Value::Unsigned(expected)) => Ok(actual < expected),
            (Value::Signed(actual), Value::Signed(expected)) => Ok(actual < expected),
            _ => Err(iterate_contract::Error::ComparisonTypeMismatch(
                condition.field,
            )),
        },
        ConditionOperator::Contains => match (field.value, condition.value) {
            (Value::Text(actual), Value::Text(expected)) => Ok(actual.contains(expected)),
            _ => Err(iterate_contract::Error::ComparisonTypeMismatch(
                condition.field,
            )),
        },
        ConditionOperator::StartsWith => match (field.value, condition.value) {
            (Value::Text(actual), Value::Text(expected)) => Ok(actual.starts_with(expected)),
            _ => Err(iterate_contract::Error::ComparisonTypeMismatch(
                condition.field,
            )),
        },
        ConditionOperator::EndsWith => match (field.value, condition.value) {
            (Value::Text(actual), Value::Text(expected)) => Ok(actual.ends_with(expected)),
            _ => Err(iterate_contract::Error::ComparisonTypeMismatch(
                condition.field,
            )),
        },
    }
}

fn matches_between<'iteration>(
    between: &BetweenCondition<'iteration>,
    record: &Record<'_>,
) -> Result<bool, iterate_contract::Error<'iteration>> {
    let field = find_field(record, between.field)
        .ok_or(iterate_contract::Error::FieldNotFound(between.field))?;

    match (field.value, between.lower, between.upper) {
        (Value::Unsigned(actual), Value::Unsigned(lower), Value::Unsigned(upper)) => {
            Ok(lower <= actual && actual <= upper)
        }
        (Value::Signed(actual), Value::Signed(lower), Value::Signed(upper)) => {
            Ok(lower <= actual && actual <= upper)
        }
        _ => Err(iterate_contract::Error::ComparisonTypeMismatch(
            between.field,
        )),
    }
}

fn matches_in<'iteration>(
    in_condition: &InCondition<'iteration>,
    record: &Record<'_>,
) -> Result<bool, iterate_contract::Error<'iteration>> {
    let field = find_field(record, in_condition.field)
        .ok_or(iterate_contract::Error::FieldNotFound(in_condition.field))?;

    if in_condition.values.is_empty() {
        return Ok(false);
    }

    let mut is_matched = false;
    for value in in_condition.values {
        match (field.value, *value) {
            (Value::Text(actual), Value::Text(candidate)) => {
                if actual == candidate {
                    is_matched = true;
                }
            }
            (Value::Unsigned(actual), Value::Unsigned(candidate)) => {
                if actual == candidate {
                    is_matched = true;
                }
            }
            (Value::Signed(actual), Value::Signed(candidate)) => {
                if actual == candidate {
                    is_matched = true;
                }
            }
            (Value::Boolean(actual), Value::Boolean(candidate)) => {
                if actual == candidate {
                    is_matched = true;
                }
            }
            _ => {
                return Err(iterate_contract::Error::ComparisonTypeMismatch(
                    in_condition.field,
                ));
            }
        }
    }

    Ok(is_matched)
}

fn matches_expression<'iteration>(
    expression: &ConditionExpression<'iteration>,
    record: &Record<'_>,
) -> Result<bool, iterate_contract::Error<'iteration>> {
    match expression {
        ConditionExpression::Condition(condition) => matches_condition(condition, record),
        ConditionExpression::Between(between) => matches_between(between, record),
        ConditionExpression::In(in_condition) => matches_in(in_condition, record),
        ConditionExpression::Not(inner) => {
            let inner_match = matches_expression(inner, record)?;
            Ok(!inner_match)
        }
        ConditionExpression::And(expressions) => {
            for expr in *expressions {
                if !matches_expression(expr, record)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        ConditionExpression::Or(expressions) => {
            for expr in *expressions {
                if matches_expression(expr, record)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
    }
}

fn passes_operations<'iteration>(
    operations: &'iteration [IterationOperation<'iteration>],
    stage_counts: &mut [usize],
    record: &Record<'_>,
) -> Result<bool, iterate_contract::Error<'iteration>> {
    for (index, operation) in operations.iter().enumerate() {
        match operation {
            IterationOperation::Filter(expression) => {
                if !matches_expression(expression, record)? {
                    return Ok(false);
                }
            }
            IterationOperation::Skip(n) => {
                if stage_counts[index] < *n {
                    stage_counts[index] += 1;
                    return Ok(false);
                }
            }
            IterationOperation::Take(n) => {
                if stage_counts[index] < *n {
                    stage_counts[index] += 1;
                } else {
                    return Ok(false);
                }
            }
            _ => return Err(iterate_contract::Error::ProviderIncompatible),
        }
    }
    Ok(true)
}

pub fn iterate<'iteration>(
    iteration: Iteration<'iteration>,
    request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    for operation in iteration.operations {
        match operation {
            IterationOperation::Filter(_)
            | IterationOperation::Skip(_)
            | IterationOperation::Take(_) => {}
            _ => return Err(iterate_contract::Error::ProviderIncompatible),
        }
    }

    let mut stage_counts = vec![0usize; iteration.operations.len()];

    let current_dir = std::env::current_dir().map_err(|_| iterate_contract::Error::Unavailable)?;
    let read_dir =
        std::fs::read_dir(&current_dir).map_err(|_| iterate_contract::Error::Unavailable)?;

    for (index, entry_result) in read_dir.enumerate() {
        let entry = entry_result.map_err(|_| iterate_contract::Error::Unavailable)?;

        let file_name = entry.file_name();
        let name_str = file_name
            .to_str()
            .ok_or(iterate_contract::Error::ExternalTypeIncompatible("name"))?;

        let path_buf = entry.path();
        let path_str = path_buf
            .to_str()
            .ok_or(iterate_contract::Error::ExternalTypeIncompatible("path"))?;

        let file_type = entry
            .file_type()
            .map_err(|_| iterate_contract::Error::Unavailable)?;

        let kind_str = if file_type.is_file() {
            "file"
        } else if file_type.is_dir() {
            "directory"
        } else if file_type.is_symlink() {
            "symlink"
        } else {
            "other"
        };

        let index_field = Field {
            name: "index",
            value: Value::Unsigned(index as u64),
        };
        let name_field = Field {
            name: "name",
            value: Value::Text(name_str),
        };
        let path_field = Field {
            name: "path",
            value: Value::Text(path_str),
        };
        let kind_field = Field {
            name: "kind",
            value: Value::Text(kind_str),
        };

        let flow = if file_type.is_file() {
            let metadata = entry
                .metadata()
                .map_err(|_| iterate_contract::Error::Unavailable)?;
            let size_field = Field {
                name: "size",
                value: Value::Unsigned(metadata.len()),
            };
            let fields = [index_field, name_field, path_field, kind_field, size_field];
            let record = Record { fields: &fields };
            if passes_operations(iteration.operations, &mut stage_counts, &record)? {
                request(Construction::Record(record))
            } else {
                Flow::Continue
            }
        } else {
            let fields = [index_field, name_field, path_field, kind_field];
            let record = Record { fields: &fields };
            if passes_operations(iteration.operations, &mut stage_counts, &record)? {
                request(Construction::Record(record))
            } else {
                Flow::Continue
            }
        };

        if flow == Flow::Stop {
            return Ok(());
        }
    }

    Ok(())
}

pub const ITERATE: iterate_contract::Iterate = iterate;
