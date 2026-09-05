use crate::definitions::failures::ComparisonFailure;
use crate::definitions::value::{OwnedValue, Value};

pub type Greater =
    for<'left, 'right> fn(&Value<'left>, &Value<'right>) -> Result<bool, ComparisonFailure>;

pub type OwnedGreater = fn(&OwnedValue, &OwnedValue) -> Result<bool, ComparisonFailure>;
