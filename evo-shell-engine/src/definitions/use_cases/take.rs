use crate::definitions::domain::value_objects::structured_items::StructuredItems;

pub type Take = for<'a> fn(items: StructuredItems<'a>, count: usize) -> StructuredItems<'a>;
