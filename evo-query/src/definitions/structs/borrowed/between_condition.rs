use evo_values::definitions::value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BetweenCondition<'condition> {
    pub field: &'condition str,
    pub lower: Value<'condition>,
    pub upper: Value<'condition>,
}
