use super::between_condition::BetweenCondition;
use super::condition::Condition;
use super::in_condition::InCondition;

#[derive(Debug, Clone, PartialEq)]
pub enum ConditionExpression<'condition> {
    Condition(Condition<'condition>),
    Between(BetweenCondition<'condition>),
    In(InCondition<'condition>),
    Not(&'condition ConditionExpression<'condition>),
    And(&'condition [ConditionExpression<'condition>]),
    Or(&'condition [ConditionExpression<'condition>]),
}
