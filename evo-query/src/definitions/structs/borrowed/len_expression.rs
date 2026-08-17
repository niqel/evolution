use super::value_expression::ValueExpression;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LenExpression<'expression> {
    pub text: &'expression ValueExpression<'expression>,
}
