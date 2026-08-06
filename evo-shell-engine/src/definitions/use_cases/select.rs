use crate::definitions::domain::entities::filesystem_iteration_item::FilesystemIterationItem;
use crate::definitions::domain::value_objects::select::{SelectProperty, StructuredProjection};

pub type Select = fn(
    items: &[FilesystemIterationItem],
    properties: &[SelectProperty],
) -> Result<StructuredProjection, SelectError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectError {
    UnsupportedProperty(String),
}
