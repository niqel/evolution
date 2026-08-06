use crate::definitions::domain::value_objects::select::ProjectedValue;
use crate::definitions::domain::value_objects::select::StructuredProjection;

pub type ToValue = fn(projection: StructuredProjection) -> Result<ProjectedValue, ToValueError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToValueError {
    NoRows,
    MultipleRows { actual: usize },
    NoProperties,
    MultipleProperties { actual: usize },
    InconsistentRowWidth { row: usize, actual: usize },
}
