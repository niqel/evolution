use crate::definitions::domain::value_objects::arguments::Arguments;
use crate::definitions::domain::value_objects::select::StructuredProjection;

pub type ToArgs = fn(projection: StructuredProjection) -> Result<Arguments, ToArgsError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToArgsError {
    InvalidPropertyCount { actual: usize },
    InconsistentRowWidth { row: usize, actual: usize },
}
