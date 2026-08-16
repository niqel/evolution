#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionOperator {
    Equal,
    GreaterThan,
    LessThan,
    Contains,
    StartsWith,
    EndsWith,
}
