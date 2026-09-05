use crate::definitions::failures::ComparisonFailure;
use crate::definitions::value::{OwnedValue, Value};

pub type GreaterEqual =
    for<'left, 'right> fn(&Value<'left>, &Value<'right>) -> Result<bool, ComparisonFailure>;

pub type OwnedGreaterEqual = fn(&OwnedValue, &OwnedValue) -> Result<bool, ComparisonFailure>;
