use super::value_expression::ValueExpression;

#[derive(Debug, Clone, PartialEq)]
pub struct NewField<'field> {
    pub name: &'field str,
    pub expression: ValueExpression<'field>,
}
