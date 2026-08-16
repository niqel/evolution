use super::condition_expression::ConditionExpression;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IterationOperation<'operation> {
    Filter(ConditionExpression<'operation>),
    Select(&'operation [&'operation str]),
    ToValue,
    Take(usize),
    Skip(usize),
    First,
    Last,
    Count,
    Iter,
}
