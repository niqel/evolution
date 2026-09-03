use super::value_expression::ValueExpression;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReplaceExpression<'expression> {
    pub text: &'expression ValueExpression<'expression>,
    pub from: &'expression ValueExpression<'expression>,
    pub to: &'expression ValueExpression<'expression>,
}
