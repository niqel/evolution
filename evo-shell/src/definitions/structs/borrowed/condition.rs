use super::value::Value;
use crate::definitions::structs::owned::condition_operator::ConditionOperator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Condition<'condition> {
    pub field: &'condition str,
    pub operator: ConditionOperator,
    pub value: Value<'condition>,
}
