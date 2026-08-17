use super::condition_expression::ConditionExpression;
use super::selection::Selection;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IterationOperation<'operation> {
    Filter(ConditionExpression<'operation>),
    Select(&'operation [Selection<'operation>]),
    ToValue,
    Take(usize),
    Skip(usize),
    First,
    Last,
    Count,
}
