use super::value_expression::ValueExpression;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LenExpression<'expression> {
    pub text: &'expression ValueExpression<'expression>,
}
