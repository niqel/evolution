use crate::definitions::failures::ComparisonFailure;
use crate::definitions::value::{OwnedValue, Value};

pub type Equal =
    for<'left, 'right> fn(&Value<'left>, &Value<'right>) -> Result<bool, ComparisonFailure>;

pub type OwnedEqual = fn(&OwnedValue, &OwnedValue) -> Result<bool, ComparisonFailure>;
