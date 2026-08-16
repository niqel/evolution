use super::between_condition::BetweenCondition;
use super::condition::Condition;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionExpression<'condition> {
    Condition(Condition<'condition>),
    Between(BetweenCondition<'condition>),
    And(&'condition [ConditionExpression<'condition>]),
    Or(&'condition [ConditionExpression<'condition>]),
}
