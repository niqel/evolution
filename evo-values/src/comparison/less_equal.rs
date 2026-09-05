use crate::comparison::kernel::{ComparisonOp, compare_owned_value, compare_value};
use crate::definitions::comparison::less_equal as less_equal_definition;
use crate::definitions::failures::ComparisonFailure;
use crate::definitions::value::{OwnedValue, Value};

pub fn less_equal(left: &Value<'_>, right: &Value<'_>) -> Result<bool, ComparisonFailure> {
    compare_value(ComparisonOp::LessEqual, left, right)
}

pub const LESS_EQUAL: less_equal_definition::LessEqual = less_equal;

pub fn owned_less_equal(left: &OwnedValue, right: &OwnedValue) -> Result<bool, ComparisonFailure> {
    compare_owned_value(ComparisonOp::LessEqual, left, right)
}

pub const OWNED_LESS_EQUAL: less_equal_definition::OwnedLessEqual = owned_less_equal;
