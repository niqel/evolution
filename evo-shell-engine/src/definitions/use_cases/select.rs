use crate::definitions::domain::value_objects::select::{SelectProperty, StructuredProjection};
use crate::definitions::domain::value_objects::structured_items::StructuredItems;

pub type Select = for<'a> fn(
    items: StructuredItems<'a>,
    properties: &[SelectProperty],
) -> Result<StructuredProjection, SelectError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectError {
    UnsupportedProperty(String),
}
