use crate::definitions::domain::value_objects::select::StructuredProjection;
use crate::definitions::domain::value_objects::values::Values;
use crate::definitions::use_cases::to_values::ToValuesError;
use crate::resolvers::to_values;

pub fn convert(projection: StructuredProjection) -> Result<Values, ToValuesError> {
    to_values::resolve(projection)
}
