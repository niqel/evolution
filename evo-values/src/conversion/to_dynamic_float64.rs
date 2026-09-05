use crate::definitions::conversion::to_dynamic_float64::ToDynamicFloat64;
use crate::definitions::value::OwnedDynamicValue;

pub fn to_dynamic_float64_from_f64(source: f64) -> OwnedDynamicValue {
    OwnedDynamicValue::Float64(source)
}
pub const TO_DYNAMIC_FLOAT64_FROM_F64: ToDynamicFloat64 = to_dynamic_float64_from_f64;
