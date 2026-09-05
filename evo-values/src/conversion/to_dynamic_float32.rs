use crate::definitions::conversion::to_dynamic_float32::ToDynamicFloat32;
use crate::definitions::value::OwnedDynamicValue;

pub fn to_dynamic_float32_from_f32(source: f32) -> OwnedDynamicValue {
    OwnedDynamicValue::Float32(source)
}
pub const TO_DYNAMIC_FLOAT32_FROM_F32: ToDynamicFloat32 = to_dynamic_float32_from_f32;
