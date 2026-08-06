use crate::definitions::domain::value_objects::structured_items::StructuredItems;
use crate::definitions::use_cases::index::IndexError;
use crate::resolvers::index;

pub fn index<'a>(
    items: StructuredItems<'a>,
    index: usize,
) -> Result<StructuredItems<'a>, IndexError> {
    index::resolve(items, index)
}
