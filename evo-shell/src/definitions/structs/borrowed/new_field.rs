use super::value_expression::ValueExpression;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NewField<'field> {
    pub name: &'field str,
    pub expression: ValueExpression<'field>,
}
