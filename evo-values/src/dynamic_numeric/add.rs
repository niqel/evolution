use super::integer::{from_big_int, to_big_int};
use crate::definitions::dynamic_numeric::DynamicAdd;
use crate::definitions::failures::DynamicNumericFailure;
use crate::definitions::value::{DynamicValue, OwnedDynamicValue};

pub fn dynamic_add<'left, 'right>(
    left: &DynamicValue<'left>,
    right: &DynamicValue<'right>,
) -> Result<OwnedDynamicValue, DynamicNumericFailure> {
    match (left, right) {
        (DynamicValue::Integer(l), DynamicValue::Integer(r)) => {
            let left_big = to_big_int(l);
            let right_big = to_big_int(r);
            let sum = left_big + right_big;
            Ok(OwnedDynamicValue::Integer(from_big_int(sum)))
        }
        (DynamicValue::Float32(l), DynamicValue::Float32(r)) => {
            Ok(OwnedDynamicValue::Float32(l + r))
        }
        (DynamicValue::Float64(l), DynamicValue::Float64(r)) => {
            Ok(OwnedDynamicValue::Float64(l + r))
        }
        _ => Err(DynamicNumericFailure::DifferentFamily),
    }
}

pub const DYNAMIC_ADD: DynamicAdd = dynamic_add;
