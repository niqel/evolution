use crate::definitions::failures::ComparisonFailure;
use crate::definitions::value::{OwnedValue, Value};

pub type Less =
    for<'left, 'right> fn(&Value<'left>, &Value<'right>) -> Result<bool, ComparisonFailure>;

pub type OwnedLess = fn(&OwnedValue, &OwnedValue) -> Result<bool, ComparisonFailure>;
