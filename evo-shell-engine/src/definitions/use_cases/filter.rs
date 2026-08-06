use crate::definitions::domain::value_objects::filter::FilterExpression;
use crate::definitions::domain::value_objects::structured_items::StructuredItems;

pub type Filter = for<'a> fn(
    items: StructuredItems<'a>,
    expression: &FilterExpression,
) -> Result<StructuredItems<'a>, FilterError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterError {
    UnsupportedProperty(String),
    InvalidOperatorForProperty {
        property: crate::definitions::domain::value_objects::filter::FilterProperty,
        operator: crate::definitions::domain::value_objects::filter::FilterOperator,
    },
    MissingComparableValue {
        property: crate::definitions::domain::value_objects::filter::FilterProperty,
    },
}
