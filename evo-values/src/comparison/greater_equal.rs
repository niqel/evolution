use crate::comparison::kernel::{ComparisonOp, compare_owned_value, compare_value};
use crate::definitions::comparison::greater_equal as greater_equal_definition;
use crate::definitions::failures::ComparisonFailure;
use crate::definitions::value::{OwnedValue, Value};

pub fn greater_equal(left: &Value<'_>, right: &Value<'_>) -> Result<bool, ComparisonFailure> {
    compare_value(ComparisonOp::GreaterEqual, left, right)
}

pub const GREATER_EQUAL: greater_equal_definition::GreaterEqual = greater_equal;

pub fn owned_greater_equal(
    left: &OwnedValue,
    right: &OwnedValue,
) -> Result<bool, ComparisonFailure> {
    compare_owned_value(ComparisonOp::GreaterEqual, left, right)
}

pub const OWNED_GREATER_EQUAL: greater_equal_definition::OwnedGreaterEqual = owned_greater_equal;
