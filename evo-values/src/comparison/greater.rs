use crate::comparison::kernel::{ComparisonOp, compare_owned_value, compare_value};
use crate::definitions::comparison::greater as greater_definition;
use crate::definitions::failures::ComparisonFailure;
use crate::definitions::value::{OwnedValue, Value};

pub fn greater(left: &Value<'_>, right: &Value<'_>) -> Result<bool, ComparisonFailure> {
    compare_value(ComparisonOp::Greater, left, right)
}

pub const GREATER: greater_definition::Greater = greater;

pub fn owned_greater(left: &OwnedValue, right: &OwnedValue) -> Result<bool, ComparisonFailure> {
    compare_owned_value(ComparisonOp::Greater, left, right)
}

pub const OWNED_GREATER: greater_definition::OwnedGreater = owned_greater;
