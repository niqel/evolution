use evo_query::definitions::contracts::iterate as iterate_contract;
use evo_query::definitions::requesters::construction_requester;
use evo_query::definitions::structs::borrowed::between_condition::BetweenCondition;
use evo_query::definitions::structs::borrowed::condition::Condition;
use evo_query::definitions::structs::borrowed::condition_expression::ConditionExpression;
use evo_query::definitions::structs::borrowed::construction::Construction;
use evo_query::definitions::structs::borrowed::field::Field;
use evo_query::definitions::structs::borrowed::in_condition::InCondition;
use evo_query::definitions::structs::borrowed::iteration::Iteration;
use evo_query::definitions::structs::borrowed::iteration_operation::IterationOperation;
use evo_query::definitions::structs::borrowed::record::Record;
use evo_query::definitions::structs::borrowed::selection::Selection;
use evo_query::definitions::structs::borrowed::value_expression::ValueExpression;
use evo_query::definitions::structs::owned::condition_operator::ConditionOperator;
use evo_query::definitions::structs::owned::flow::Flow;
use evo_values::definitions::value::Value;
use evo_values::text::concat::CONCAT;
use evo_values::text::len::LEN;
use evo_values::text::replace::REPLACE;
use evo_values::text::substring::SUBSTRING;

#[derive(Clone, Debug)]
enum OwnedValue {
    String(String),
    Uint64(u64),
    Int64(i64),
    Boolean(bool),
}

impl OwnedValue {
    fn from_borrowed(value: &Value<'_>) -> Self {
        match value {
            Value::String(t) => OwnedValue::String(t.to_string()),
            Value::Uint64(u) => OwnedValue::Uint64(*u),
            Value::Int64(s) => OwnedValue::Int64(*s),
            Value::Boolean(b) => OwnedValue::Boolean(*b),
            _ => panic!("unsupported value family in fs provider"),
        }
    }

    fn as_borrowed(&self) -> Value<'_> {
        match self {
            OwnedValue::String(t) => Value::String(t.as_str()),
            OwnedValue::Uint64(u) => Value::Uint64(*u),
            OwnedValue::Int64(s) => Value::Int64(*s),
            OwnedValue::Boolean(b) => Value::Boolean(*b),
        }
    }
}

#[derive(Clone, Debug)]
struct OwnedField {
    name: String,
    value: OwnedValue,
}

impl OwnedField {
    fn from_borrowed(field: &Field<'_>) -> Self {
        OwnedField {
            name: field.name.to_string(),
            value: OwnedValue::from_borrowed(&field.value),
        }
    }

    fn as_borrowed(&self) -> Field<'_> {
        Field {
            name: self.name.as_str(),
            value: self.value.as_borrowed(),
        }
    }
}

#[derive(Clone, Debug)]
enum OwnedConstruction {
    Record(Vec<OwnedField>),
    Value(OwnedValue),
}

impl OwnedConstruction {
    fn from_construction(construction: Construction<'_>) -> Self {
        match construction {
            Construction::Record(record) => {
                let fields = record
                    .fields
                    .iter()
                    .map(OwnedField::from_borrowed)
                    .collect();
                OwnedConstruction::Record(fields)
            }
            Construction::Value(val) => OwnedConstruction::Value(OwnedValue::from_borrowed(&val)),
        }
    }
}

#[derive(Clone, Debug)]
enum EvaluatedValue<'item> {
    Borrowed(Value<'item>),
    Owned(OwnedValue),
}

impl<'item> EvaluatedValue<'item> {
    fn as_borrowed(&self) -> Value<'_> {
        match self {
            EvaluatedValue::Borrowed(v) => v.clone(),
            EvaluatedValue::Owned(owned) => owned.as_borrowed(),
        }
    }
}

struct EvaluatedField<'item> {
    name: &'item str,
    value: EvaluatedValue<'item>,
}

enum StageState {
    Filter,
    Select,
    ToValue,
    Skip { count: usize },
    Take { count: usize },
    First { seen: bool },
    Last { item: Option<OwnedConstruction> },
    Count { count: u64 },
}

fn find_field<'a, 'field>(
    record: &'a Record<'field>,
    field_name: &str,
) -> Option<&'a Field<'field>> {
    record.fields.iter().find(|f| f.name == field_name)
}

fn matches_condition<'iteration>(
    condition: &Condition<'iteration>,
    record: &Record<'_>,
) -> Result<bool, iterate_contract::Error<'iteration>> {
    let field = find_field(record, condition.field)
        .ok_or(iterate_contract::Error::FieldNotFound(condition.field))?;

    match condition.operator {
        ConditionOperator::Equal => match (&field.value, &condition.value) {
            (Value::String(actual), Value::String(expected)) => Ok(actual == expected),
            (Value::Uint64(actual), Value::Uint64(expected)) => Ok(actual == expected),
            (Value::Int64(actual), Value::Int64(expected)) => Ok(actual == expected),
            (Value::Boolean(actual), Value::Boolean(expected)) => Ok(actual == expected),
            _ => Err(iterate_contract::Error::ComparisonTypeMismatch(
                condition.field,
            )),
        },
        ConditionOperator::GreaterThan => match (&field.value, &condition.value) {
            (Value::Uint64(actual), Value::Uint64(expected)) => Ok(actual > expected),
            (Value::Int64(actual), Value::Int64(expected)) => Ok(actual > expected),
            _ => Err(iterate_contract::Error::ComparisonTypeMismatch(
                condition.field,
            )),
        },
        ConditionOperator::LessThan => match (&field.value, &condition.value) {
            (Value::Uint64(actual), Value::Uint64(expected)) => Ok(actual < expected),
            (Value::Int64(actual), Value::Int64(expected)) => Ok(actual < expected),
            _ => Err(iterate_contract::Error::ComparisonTypeMismatch(
                condition.field,
            )),
        },
        ConditionOperator::Contains => match (&field.value, &condition.value) {
            (Value::String(actual), Value::String(expected)) => Ok(actual.contains(expected)),
            _ => Err(iterate_contract::Error::ComparisonTypeMismatch(
                condition.field,
            )),
        },
        ConditionOperator::StartsWith => match (&field.value, &condition.value) {
            (Value::String(actual), Value::String(expected)) => Ok(actual.starts_with(expected)),
            _ => Err(iterate_contract::Error::ComparisonTypeMismatch(
                condition.field,
            )),
        },
        ConditionOperator::EndsWith => match (&field.value, &condition.value) {
            (Value::String(actual), Value::String(expected)) => Ok(actual.ends_with(expected)),
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

    match (&field.value, &between.lower, &between.upper) {
        (Value::Uint64(actual), Value::Uint64(lower), Value::Uint64(upper)) => {
            Ok(lower <= actual && actual <= upper)
        }
        (Value::Int64(actual), Value::Int64(lower), Value::Int64(upper)) => {
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
        match (&field.value, value) {
            (Value::String(actual), Value::String(candidate)) => {
                if actual == candidate {
                    is_matched = true;
                }
            }
            (Value::Uint64(actual), Value::Uint64(candidate)) => {
                if actual == candidate {
                    is_matched = true;
                }
            }
            (Value::Int64(actual), Value::Int64(candidate)) => {
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

fn evaluate_value_pipeline<'iteration, 'item>(
    operations: &'iteration [IterationOperation<'iteration>],
    input_fields: &[Field<'item>],
) -> Result<Value<'item>, iterate_contract::Error<'iteration>>
where
    'iteration: 'item,
{
    if operations.is_empty() {
        return Err(iterate_contract::Error::ProviderIncompatible);
    }

    let mut projected_fields: Option<Vec<Field<'item>>> = None;
    let mut current_value: Option<Value<'item>> = None;

    for operation in operations {
        match operation {
            IterationOperation::Select(selections) => {
                if current_value.is_some() || selections.is_empty() {
                    return Err(iterate_contract::Error::ProviderIncompatible);
                }
                let current_slice: &[Field<'item>] = match &projected_fields {
                    Some(fields) => fields.as_slice(),
                    None => input_fields,
                };
                let mut next_fields = Vec::with_capacity(selections.len());
                for selection in *selections {
                    match selection {
                        Selection::Field(field_name) => {
                            let field = current_slice
                                .iter()
                                .find(|f| f.name == *field_name)
                                .cloned()
                                .ok_or(iterate_contract::Error::FieldNotFound(field_name))?;
                            next_fields.push(field);
                        }
                        Selection::New(_) => {
                            return Err(iterate_contract::Error::ProviderIncompatible);
                        }
                    }
                }
                projected_fields = Some(next_fields);
            }
            IterationOperation::ToValue => {
                if current_value.is_some() {
                    return Err(iterate_contract::Error::ToValueRequiresRecord);
                }
                let current_slice: &[Field<'item>] = match &projected_fields {
                    Some(fields) => fields.as_slice(),
                    None => input_fields,
                };
                if current_slice.len() != 1 {
                    return Err(iterate_contract::Error::ToValueRequiresSingleField);
                }
                current_value = Some(current_slice[0].value.clone());
            }
            _ => return Err(iterate_contract::Error::ProviderIncompatible),
        }
    }

    current_value.ok_or(iterate_contract::Error::ProviderIncompatible)
}

fn evaluate_value_expression<'iteration, 'item>(
    expression: &'iteration ValueExpression<'iteration>,
    input_fields: &[Field<'item>],
) -> Result<EvaluatedValue<'item>, iterate_contract::Error<'iteration>>
where
    'iteration: 'item,
{
    match expression {
        ValueExpression::Literal(value) => Ok(EvaluatedValue::Borrowed(value.clone())),
        ValueExpression::Pipeline(operations) => {
            let val = evaluate_value_pipeline(operations, input_fields)?;
            Ok(EvaluatedValue::Borrowed(val))
        }
        ValueExpression::Concat(expressions) => {
            let mut temp_evaluated: Vec<EvaluatedValue<'item>> =
                Vec::with_capacity(expressions.len());
            for expr in *expressions {
                let eval = evaluate_value_expression(expr, input_fields)?;
                temp_evaluated.push(eval);
            }
            let mut parts: Vec<&str> = Vec::with_capacity(temp_evaluated.len());
            for eval in &temp_evaluated {
                match eval.as_borrowed() {
                    Value::String(t) => parts.push(t),
                    _ => return Err(iterate_contract::Error::TextExpected),
                }
            }
            let result = CONCAT(&parts);
            Ok(EvaluatedValue::Owned(OwnedValue::String(result)))
        }
        ValueExpression::Len(len_expr) => {
            let eval = evaluate_value_expression(len_expr.text, input_fields)?;
            match eval.as_borrowed() {
                Value::String(t) => {
                    let count = LEN(t);
                    Ok(EvaluatedValue::Borrowed(Value::Uint64(count as u64)))
                }
                _ => Err(iterate_contract::Error::TextExpected),
            }
        }
        ValueExpression::Substring(substring_expr) => {
            let text_eval = evaluate_value_expression(substring_expr.text, input_fields)?;
            let start_eval = evaluate_value_expression(substring_expr.start, input_fields)?;
            let length_eval = evaluate_value_expression(substring_expr.length, input_fields)?;

            let start_u64 = match start_eval.as_borrowed() {
                Value::Uint64(u) => u,
                _ => return Err(iterate_contract::Error::UnsignedExpected),
            };
            let length_u64 = match length_eval.as_borrowed() {
                Value::Uint64(u) => u,
                _ => return Err(iterate_contract::Error::UnsignedExpected),
            };

            let start_usize = usize::try_from(start_u64)
                .map_err(|_| iterate_contract::Error::SubstringOutOfBounds)?;
            let length_usize = usize::try_from(length_u64)
                .map_err(|_| iterate_contract::Error::SubstringOutOfBounds)?;

            match text_eval {
                EvaluatedValue::Borrowed(Value::String(s)) => {
                    let slice = SUBSTRING(s, start_usize, length_usize)
                        .map_err(|_| iterate_contract::Error::SubstringOutOfBounds)?;
                    Ok(EvaluatedValue::Borrowed(Value::String(slice)))
                }
                EvaluatedValue::Owned(OwnedValue::String(ref s)) => {
                    let slice = SUBSTRING(s.as_str(), start_usize, length_usize)
                        .map_err(|_| iterate_contract::Error::SubstringOutOfBounds)?;
                    Ok(EvaluatedValue::Owned(OwnedValue::String(slice.to_string())))
                }
                _ => Err(iterate_contract::Error::TextExpected),
            }
        }
        ValueExpression::Replace(replace_expr) => {
            let text_eval = evaluate_value_expression(replace_expr.text, input_fields)?;
            let from_eval = evaluate_value_expression(replace_expr.from, input_fields)?;
            let to_eval = evaluate_value_expression(replace_expr.to, input_fields)?;

            let text_str = match text_eval.as_borrowed() {
                Value::String(s) => s,
                _ => return Err(iterate_contract::Error::TextExpected),
            };
            let from_str = match from_eval.as_borrowed() {
                Value::String(s) => s,
                _ => return Err(iterate_contract::Error::TextExpected),
            };
            let to_str = match to_eval.as_borrowed() {
                Value::String(s) => s,
                _ => return Err(iterate_contract::Error::TextExpected),
            };

            let replaced = REPLACE(text_str, from_str, to_str).map_err(|e| match e {
                evo_values::definitions::text::replace::Error::EmptyPattern => {
                    iterate_contract::Error::ReplaceEmptyPattern
                }
            })?;

            Ok(EvaluatedValue::Owned(OwnedValue::String(replaced)))
        }
    }
}

fn process_item_from<'iteration, 'item>(
    stage_index: usize,
    operations: &'iteration [IterationOperation<'iteration>],
    stages: &mut [StageState],
    construction: Construction<'item>,
    request: construction_requester::Request,
) -> Result<Flow, iterate_contract::Error<'iteration>>
where
    'iteration: 'item,
{
    if stage_index >= operations.len() {
        return Ok(request(construction));
    }

    match &operations[stage_index] {
        IterationOperation::Filter(expression) => match construction {
            Construction::Record(record) => {
                if !matches_expression(expression, &record)? {
                    return Ok(Flow::Continue);
                }
                process_item_from(
                    stage_index + 1,
                    operations,
                    stages,
                    Construction::Record(record),
                    request,
                )
            }
            Construction::Value(_) => Err(iterate_contract::Error::ProviderIncompatible),
        },
        IterationOperation::Select(selections) => match construction {
            Construction::Record(record) => {
                let mut evaluated_fields = Vec::with_capacity(selections.len());
                for selection in *selections {
                    match selection {
                        Selection::Field(field_name) => {
                            let field = record
                                .fields
                                .iter()
                                .find(|f| f.name == *field_name)
                                .ok_or(iterate_contract::Error::FieldNotFound(field_name))?;
                            evaluated_fields.push(EvaluatedField {
                                name: field.name,
                                value: EvaluatedValue::Borrowed(field.value.clone()),
                            });
                        }
                        Selection::New(new_field) => {
                            let evaluated_val =
                                evaluate_value_expression(&new_field.expression, record.fields)?;
                            evaluated_fields.push(EvaluatedField {
                                name: new_field.name,
                                value: evaluated_val,
                            });
                        }
                    }
                }
                let borrowed_fields: Vec<Field<'_>> = evaluated_fields
                    .iter()
                    .map(|ef| Field {
                        name: ef.name,
                        value: ef.value.as_borrowed(),
                    })
                    .collect();
                let next_record = Record {
                    fields: &borrowed_fields,
                };
                process_item_from(
                    stage_index + 1,
                    operations,
                    stages,
                    Construction::Record(next_record),
                    request,
                )
            }
            Construction::Value(_) => Err(iterate_contract::Error::ProviderIncompatible),
        },
        IterationOperation::ToValue => match construction {
            Construction::Record(record) => {
                if record.fields.len() != 1 {
                    return Err(iterate_contract::Error::ToValueRequiresSingleField);
                }
                let value = record.fields[0].value.clone();
                process_item_from(
                    stage_index + 1,
                    operations,
                    stages,
                    Construction::Value(value),
                    request,
                )
            }
            Construction::Value(_) => Err(iterate_contract::Error::ToValueRequiresRecord),
        },
        IterationOperation::Skip(n) => {
            let should_skip = match &mut stages[stage_index] {
                StageState::Skip { count } => {
                    if *count < *n {
                        *count += 1;
                        true
                    } else {
                        false
                    }
                }
                _ => unreachable!(),
            };
            if should_skip {
                Ok(Flow::Continue)
            } else {
                process_item_from(stage_index + 1, operations, stages, construction, request)
            }
        }
        IterationOperation::Take(n) => {
            let should_take = match &mut stages[stage_index] {
                StageState::Take { count } => {
                    if *count < *n {
                        *count += 1;
                        true
                    } else {
                        false
                    }
                }
                _ => unreachable!(),
            };
            if should_take {
                process_item_from(stage_index + 1, operations, stages, construction, request)
            } else {
                Ok(Flow::Continue)
            }
        }
        IterationOperation::First => {
            let is_first = match &mut stages[stage_index] {
                StageState::First { seen } => {
                    if !*seen {
                        *seen = true;
                        true
                    } else {
                        false
                    }
                }
                _ => unreachable!(),
            };
            if is_first {
                process_item_from(stage_index + 1, operations, stages, construction, request)
            } else {
                Ok(Flow::Continue)
            }
        }
        IterationOperation::Last => {
            match &mut stages[stage_index] {
                StageState::Last { item } => {
                    *item = Some(OwnedConstruction::from_construction(construction));
                }
                _ => unreachable!(),
            }
            Ok(Flow::Continue)
        }
        IterationOperation::Count => {
            match &mut stages[stage_index] {
                StageState::Count { count } => {
                    *count += 1;
                }
                _ => unreachable!(),
            }
            Ok(Flow::Continue)
        }
    }
}

fn finalize_from<'iteration>(
    stage_index: usize,
    operations: &'iteration [IterationOperation<'iteration>],
    stages: &mut [StageState],
    request: construction_requester::Request,
) -> Result<Flow, iterate_contract::Error<'iteration>> {
    if stage_index >= operations.len() {
        return Ok(Flow::Continue);
    }

    match &operations[stage_index] {
        IterationOperation::Last => {
            let item_opt = match &mut stages[stage_index] {
                StageState::Last { item } => item.take(),
                _ => unreachable!(),
            };

            if let Some(owned_item) = item_opt {
                let flow = match &owned_item {
                    OwnedConstruction::Value(v) => process_item_from(
                        stage_index + 1,
                        operations,
                        stages,
                        Construction::Value(v.as_borrowed()),
                        request,
                    )?,
                    OwnedConstruction::Record(owned_fields) => {
                        let borrowed_fields: Vec<Field<'_>> =
                            owned_fields.iter().map(|f| f.as_borrowed()).collect();
                        let record = Record {
                            fields: &borrowed_fields,
                        };
                        process_item_from(
                            stage_index + 1,
                            operations,
                            stages,
                            Construction::Record(record),
                            request,
                        )?
                    }
                };

                if flow == Flow::Stop {
                    return Ok(Flow::Stop);
                }
            }

            finalize_from(stage_index + 1, operations, stages, request)
        }
        IterationOperation::Count => {
            let count = match &stages[stage_index] {
                StageState::Count { count } => *count,
                _ => unreachable!(),
            };

            let flow = process_item_from(
                stage_index + 1,
                operations,
                stages,
                Construction::Value(Value::Uint64(count)),
                request,
            )?;

            if flow == Flow::Stop {
                return Ok(Flow::Stop);
            }

            finalize_from(stage_index + 1, operations, stages, request)
        }
        _ => finalize_from(stage_index + 1, operations, stages, request),
    }
}

fn validate_value_expression<'iteration>(
    expression: &'iteration ValueExpression<'iteration>,
) -> Result<(), iterate_contract::Error<'iteration>> {
    match expression {
        ValueExpression::Literal(_) => Ok(()),
        ValueExpression::Pipeline(operations) => {
            if operations.is_empty() {
                return Err(iterate_contract::Error::ProviderIncompatible);
            }
            let mut inner_is_value = false;
            for operation in *operations {
                match operation {
                    IterationOperation::Select(selections) => {
                        if inner_is_value || selections.is_empty() {
                            return Err(iterate_contract::Error::ProviderIncompatible);
                        }
                        for selection in *selections {
                            match selection {
                                Selection::Field(_) => {}
                                Selection::New(_) => {
                                    return Err(iterate_contract::Error::ProviderIncompatible);
                                }
                            }
                        }
                    }
                    IterationOperation::ToValue => {
                        if inner_is_value {
                            return Err(iterate_contract::Error::ToValueRequiresRecord);
                        }
                        inner_is_value = true;
                    }
                    _ => return Err(iterate_contract::Error::ProviderIncompatible),
                }
            }
            if !inner_is_value {
                return Err(iterate_contract::Error::ProviderIncompatible);
            }
            Ok(())
        }
        ValueExpression::Concat(expressions) => {
            for expr in *expressions {
                validate_value_expression(expr)?;
            }
            Ok(())
        }
        ValueExpression::Substring(sub) => {
            validate_value_expression(sub.text)?;
            validate_value_expression(sub.start)?;
            validate_value_expression(sub.length)?;
            Ok(())
        }
        ValueExpression::Len(len) => {
            validate_value_expression(len.text)?;
            Ok(())
        }
        ValueExpression::Replace(rep) => {
            validate_value_expression(rep.text)?;
            validate_value_expression(rep.from)?;
            validate_value_expression(rep.to)?;
            Ok(())
        }
    }
}

pub fn iterate<'iteration>(
    iteration: Iteration<'iteration>,
    request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    let mut is_value = false;
    for operation in iteration.operations {
        match operation {
            IterationOperation::Filter(_) => {
                if is_value {
                    return Err(iterate_contract::Error::ProviderIncompatible);
                }
            }
            IterationOperation::Select(selections) => {
                if is_value || selections.is_empty() {
                    return Err(iterate_contract::Error::ProviderIncompatible);
                }
                for selection in *selections {
                    match selection {
                        Selection::Field(_) => {}
                        Selection::New(new_field) => {
                            validate_value_expression(&new_field.expression)?;
                        }
                    }
                }
            }
            IterationOperation::Skip(_)
            | IterationOperation::Take(_)
            | IterationOperation::First
            | IterationOperation::Last => {}
            IterationOperation::ToValue => {
                if is_value {
                    return Err(iterate_contract::Error::ToValueRequiresRecord);
                }
                is_value = true;
            }
            IterationOperation::Count => {
                is_value = true;
            }
        }
    }

    let mut stages: Vec<StageState> = iteration
        .operations
        .iter()
        .map(|op| match op {
            IterationOperation::Filter(_) => StageState::Filter,
            IterationOperation::Select(_) => StageState::Select,
            IterationOperation::ToValue => StageState::ToValue,
            IterationOperation::Skip(_) => StageState::Skip { count: 0 },
            IterationOperation::Take(_) => StageState::Take { count: 0 },
            IterationOperation::First => StageState::First { seen: false },
            IterationOperation::Last => StageState::Last { item: None },
            IterationOperation::Count => StageState::Count { count: 0 },
        })
        .collect();

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
            value: Value::Uint64(index as u64),
        };
        let name_field = Field {
            name: "name",
            value: Value::String(name_str),
        };
        let path_field = Field {
            name: "path",
            value: Value::String(path_str),
        };
        let kind_field = Field {
            name: "kind",
            value: Value::String(kind_str),
        };

        let flow = if file_type.is_file() {
            let metadata = entry
                .metadata()
                .map_err(|_| iterate_contract::Error::Unavailable)?;
            let size_field = Field {
                name: "size",
                value: Value::Uint64(metadata.len()),
            };
            let fields = [index_field, name_field, path_field, kind_field, size_field];
            let record = Record { fields: &fields };
            process_item_from(
                0,
                iteration.operations,
                &mut stages,
                Construction::Record(record),
                request,
            )?
        } else {
            let fields = [index_field, name_field, path_field, kind_field];
            let record = Record { fields: &fields };
            process_item_from(
                0,
                iteration.operations,
                &mut stages,
                Construction::Record(record),
                request,
            )?
        };

        if flow == Flow::Stop {
            return Ok(());
        }
    }

    finalize_from(0, iteration.operations, &mut stages, request)?;

    Ok(())
}

pub const ITERATE: iterate_contract::Iterate = iterate;
