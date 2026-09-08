use alloc::boxed::Box;

use crate::definitions::dynamic_numeric::DynamicNegate;
use crate::definitions::value::{DynamicValue, OwnedDynamicInteger, OwnedDynamicValue};

pub fn dynamic_negate<'value>(value: &DynamicValue<'value>) -> OwnedDynamicValue {
    match value {
        DynamicValue::Integer(val) => {
            let magnitude = val.magnitude();
            if magnitude.is_empty() {
                OwnedDynamicValue::Integer(OwnedDynamicInteger::from_parts(false, Box::new([])))
            } else {
                let negative = !val.negative();
                OwnedDynamicValue::Integer(OwnedDynamicInteger::from_parts(
                    negative,
                    Box::from(magnitude),
                ))
            }
        }
        DynamicValue::Float32(val) => OwnedDynamicValue::Float32(-val),
        DynamicValue::Float64(val) => OwnedDynamicValue::Float64(-val),
    }
}

pub const DYNAMIC_NEGATE: DynamicNegate = dynamic_negate;
