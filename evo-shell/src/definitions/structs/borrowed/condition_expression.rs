use super::between_condition::BetweenCondition;
use super::condition::Condition;
use super::in_condition::InCondition;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionExpression<'condition> {
    Condition(Condition<'condition>),
    Between(BetweenCondition<'condition>),
    In(InCondition<'condition>),
    NotIn(InCondition<'condition>),
    And(&'condition [ConditionExpression<'condition>]),
    Or(&'condition [ConditionExpression<'condition>]),
}
