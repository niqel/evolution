use crate::definitions::failures::ComparisonFailure;
use crate::definitions::value::{OwnedValue, Value};

pub type LessEqual =
    for<'left, 'right> fn(&Value<'left>, &Value<'right>) -> Result<bool, ComparisonFailure>;

pub type OwnedLessEqual = fn(&OwnedValue, &OwnedValue) -> Result<bool, ComparisonFailure>;
