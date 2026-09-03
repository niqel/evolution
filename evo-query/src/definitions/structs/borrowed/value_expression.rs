use super::iteration_operation::IterationOperation;
use super::len_expression::LenExpression;
use super::replace_expression::ReplaceExpression;
use super::substring_expression::SubstringExpression;
use evo_values::definitions::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum ValueExpression<'expression> {
    Literal(Value<'expression>),
    Pipeline(&'expression [IterationOperation<'expression>]),
    Concat(&'expression [ValueExpression<'expression>]),
    Substring(SubstringExpression<'expression>),
    Len(LenExpression<'expression>),
    Replace(ReplaceExpression<'expression>),
}
