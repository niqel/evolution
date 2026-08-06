use crate::definitions::domain::value_objects::select::{SelectProperty, StructuredProjection};
use crate::definitions::domain::value_objects::structured_items::StructuredItems;
use crate::definitions::use_cases::select::SelectError;
use crate::resolvers::select;

pub fn select<'a>(
    items: StructuredItems<'a>,
    properties: &[SelectProperty],
) -> Result<StructuredProjection, SelectError> {
    select::resolve(items, properties)
}
