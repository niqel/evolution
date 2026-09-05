use crate::comparison::kernel::{ComparisonOp, compare_owned_value, compare_value};
use crate::definitions::comparison::not_equal as not_equal_definition;
use crate::definitions::failures::ComparisonFailure;
use crate::definitions::value::{OwnedValue, Value};

pub fn not_equal(left: &Value<'_>, right: &Value<'_>) -> Result<bool, ComparisonFailure> {
    compare_value(ComparisonOp::NotEqual, left, right)
}

pub const NOT_EQUAL: not_equal_definition::NotEqual = not_equal;

pub fn owned_not_equal(left: &OwnedValue, right: &OwnedValue) -> Result<bool, ComparisonFailure> {
    compare_owned_value(ComparisonOp::NotEqual, left, right)
}

pub const OWNED_NOT_EQUAL: not_equal_definition::OwnedNotEqual = owned_not_equal;
