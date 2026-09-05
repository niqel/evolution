use crate::comparison::kernel::{ComparisonOp, compare_owned_value, compare_value};
use crate::definitions::comparison::equal as equal_definition;
use crate::definitions::failures::ComparisonFailure;
use crate::definitions::value::{OwnedValue, Value};

pub fn equal(left: &Value<'_>, right: &Value<'_>) -> Result<bool, ComparisonFailure> {
    compare_value(ComparisonOp::Equal, left, right)
}

pub const EQUAL: equal_definition::Equal = equal;

pub fn owned_equal(left: &OwnedValue, right: &OwnedValue) -> Result<bool, ComparisonFailure> {
    compare_owned_value(ComparisonOp::Equal, left, right)
}

pub const OWNED_EQUAL: equal_definition::OwnedEqual = owned_equal;
