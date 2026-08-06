use crate::definitions::domain::value_objects::select::ProjectedValue;
use crate::definitions::domain::value_objects::select::StructuredProjection;
use crate::definitions::use_cases::to_value::ToValueError;
use crate::resolvers::to_value;

pub fn convert(projection: StructuredProjection) -> Result<ProjectedValue, ToValueError> {
    to_value::resolve(projection)
}
