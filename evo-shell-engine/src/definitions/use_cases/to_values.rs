use crate::definitions::domain::value_objects::select::StructuredProjection;
use crate::definitions::domain::value_objects::values::Values;

pub type ToValues = fn(projection: StructuredProjection) -> Result<Values, ToValuesError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToValuesError {
    InvalidPropertyCount { actual: usize },
    InconsistentRowWidth { row: usize, actual: usize },
}
