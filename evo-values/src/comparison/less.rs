use crate::comparison::kernel::{ComparisonOp, compare_owned_value, compare_value};
use crate::definitions::comparison::less as less_definition;
use crate::definitions::failures::ComparisonFailure;
use crate::definitions::value::{OwnedValue, Value};

pub fn less(left: &Value<'_>, right: &Value<'_>) -> Result<bool, ComparisonFailure> {
    compare_value(ComparisonOp::Less, left, right)
}

pub const LESS: less_definition::Less = less;

pub fn owned_less(left: &OwnedValue, right: &OwnedValue) -> Result<bool, ComparisonFailure> {
    compare_owned_value(ComparisonOp::Less, left, right)
}

pub const OWNED_LESS: less_definition::OwnedLess = owned_less;
