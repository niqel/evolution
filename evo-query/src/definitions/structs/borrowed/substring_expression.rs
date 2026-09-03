use super::value_expression::ValueExpression;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SubstringExpression<'expression> {
    pub text: &'expression ValueExpression<'expression>,
    pub start: &'expression ValueExpression<'expression>,
    pub length: &'expression ValueExpression<'expression>,
}
