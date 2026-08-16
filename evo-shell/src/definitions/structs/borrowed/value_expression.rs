use super::iteration_operation::IterationOperation;
use super::value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueExpression<'expression> {
    Literal(Value<'expression>),
    Pipeline(&'expression [IterationOperation<'expression>]),
    Concat(&'expression [ValueExpression<'expression>]),
}
