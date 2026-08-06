use crate::definitions::domain::value_objects::arguments::Arguments;
use crate::definitions::domain::value_objects::select::StructuredProjection;
use crate::definitions::use_cases::to_args::ToArgsError;
use crate::resolvers::to_args;

pub fn expand(projection: StructuredProjection) -> Result<Arguments, ToArgsError> {
    to_args::resolve(projection)
}
