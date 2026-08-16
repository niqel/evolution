use super::value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InCondition<'condition> {
    pub field: &'condition str,
    pub values: &'condition [Value<'condition>],
}
