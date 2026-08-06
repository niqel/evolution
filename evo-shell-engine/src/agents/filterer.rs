use crate::definitions::domain::entities::filesystem_iteration_item::FilesystemIterationItem;
use crate::definitions::domain::value_objects::filter::FilterExpression;
use crate::definitions::use_cases::filter::FilterError;
use crate::resolvers::filter;

pub fn filter<'a>(
    items: &'a [FilesystemIterationItem],
    expression: &FilterExpression,
) -> Result<Vec<&'a FilesystemIterationItem>, FilterError> {
    filter::resolve(items, expression)
}
