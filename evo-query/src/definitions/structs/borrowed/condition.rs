use crate::definitions::structs::owned::condition_operator::ConditionOperator;
use evo_values::definitions::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct Condition<'condition> {
    pub field: &'condition str,
    pub operator: ConditionOperator,
    pub value: Value<'condition>,
}
