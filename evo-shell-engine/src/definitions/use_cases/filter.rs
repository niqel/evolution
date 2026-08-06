use crate::definitions::domain::entities::filesystem_iteration_item::FilesystemIterationItem;
use crate::definitions::domain::value_objects::filter::FilterExpression;

pub type Filter = for<'a> fn(
    items: &'a [FilesystemIterationItem],
    expression: &FilterExpression,
) -> Result<Vec<&'a FilesystemIterationItem>, FilterError>;

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
