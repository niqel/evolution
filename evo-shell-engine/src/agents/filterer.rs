use crate::definitions::domain::value_objects::filter::FilterExpression;
use crate::definitions::domain::value_objects::structured_items::StructuredItems;
use crate::definitions::use_cases::filter::FilterError;
use crate::resolvers::filter;

pub fn filter<'a>(
    items: StructuredItems<'a>,
    expression: &FilterExpression,
) -> Result<StructuredItems<'a>, FilterError> {
    filter::resolve(items, expression)
}
