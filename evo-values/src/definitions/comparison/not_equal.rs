use crate::definitions::failures::ComparisonFailure;
use crate::definitions::value::{OwnedValue, Value};

pub type NotEqual =
    for<'left, 'right> fn(&Value<'left>, &Value<'right>) -> Result<bool, ComparisonFailure>;

pub type OwnedNotEqual = fn(&OwnedValue, &OwnedValue) -> Result<bool, ComparisonFailure>;
