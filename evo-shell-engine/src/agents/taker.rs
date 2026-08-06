use crate::definitions::domain::value_objects::structured_items::StructuredItems;
use crate::resolvers::take;

pub fn take<'a>(items: StructuredItems<'a>, count: usize) -> StructuredItems<'a> {
    take::resolve(items, count)
}
