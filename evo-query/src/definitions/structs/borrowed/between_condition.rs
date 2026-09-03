use evo_values::definitions::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub struct BetweenCondition<'condition> {
    pub field: &'condition str,
    pub lower: Value<'condition>,
    pub upper: Value<'condition>,
}
