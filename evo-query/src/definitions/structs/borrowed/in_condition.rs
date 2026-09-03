use evo_values::definitions::value::Value;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InCondition<'condition> {
    pub field: &'condition str,
    pub values: &'condition [Value<'condition>],
}
