use crate::definitions::domain::entities::filesystem_iteration_item::FilesystemIterationItem;
use crate::definitions::domain::value_objects::select::{SelectProperty, StructuredProjection};
use crate::definitions::use_cases::select::SelectError;
use crate::resolvers::select;

pub fn select(
    items: &[FilesystemIterationItem],
    properties: &[SelectProperty],
) -> Result<StructuredProjection, SelectError> {
    select::resolve(items, properties)
}
