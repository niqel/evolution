use crate::conversion::kernel::{f64_to_f32, is_u128_representable_in_f32};
use crate::definitions::conversion::to_float32::ToFloat32;
use crate::definitions::failures::ConversionFailure;

pub fn to_float32_from_i8(source: i8) -> Result<f32, ConversionFailure> {
    Ok(source as f32)
}
pub const TO_FLOAT32_FROM_I8: ToFloat32<i8> = to_float32_from_i8;

pub fn to_float32_from_i16(source: i16) -> Result<f32, ConversionFailure> {
    Ok(source as f32)
}
pub const TO_FLOAT32_FROM_I16: ToFloat32<i16> = to_float32_from_i16;

pub fn to_float32_from_i32(source: i32) -> Result<f32, ConversionFailure> {
    if is_u128_representable_in_f32(source.unsigned_abs() as u128) {
        Ok(source as f32)
    } else {
        Err(ConversionFailure::NotExactlyRepresentable)
    }
}
pub const TO_FLOAT32_FROM_I32: ToFloat32<i32> = to_float32_from_i32;

pub fn to_float32_from_i64(source: i64) -> Result<f32, ConversionFailure> {
    if is_u128_representable_in_f32(source.unsigned_abs() as u128) {
        Ok(source as f32)
    } else {
        Err(ConversionFailure::NotExactlyRepresentable)
    }
}
pub const TO_FLOAT32_FROM_I64: ToFloat32<i64> = to_float32_from_i64;

pub fn to_float32_from_i128(source: i128) -> Result<f32, ConversionFailure> {
    if is_u128_representable_in_f32(source.unsigned_abs()) {
        Ok(source as f32)
    } else {
        Err(ConversionFailure::NotExactlyRepresentable)
    }
}
pub const TO_FLOAT32_FROM_I128: ToFloat32<i128> = to_float32_from_i128;

pub fn to_float32_from_u8(source: u8) -> Result<f32, ConversionFailure> {
    Ok(source as f32)
}
pub const TO_FLOAT32_FROM_U8: ToFloat32<u8> = to_float32_from_u8;

pub fn to_float32_from_u16(source: u16) -> Result<f32, ConversionFailure> {
    Ok(source as f32)
}
pub const TO_FLOAT32_FROM_U16: ToFloat32<u16> = to_float32_from_u16;

pub fn to_float32_from_u32(source: u32) -> Result<f32, ConversionFailure> {
    if is_u128_representable_in_f32(source as u128) {
        Ok(source as f32)
    } else {
        Err(ConversionFailure::NotExactlyRepresentable)
    }
}
pub const TO_FLOAT32_FROM_U32: ToFloat32<u32> = to_float32_from_u32;

pub fn to_float32_from_u64(source: u64) -> Result<f32, ConversionFailure> {
    if is_u128_representable_in_f32(source as u128) {
        Ok(source as f32)
    } else {
        Err(ConversionFailure::NotExactlyRepresentable)
    }
}
pub const TO_FLOAT32_FROM_U64: ToFloat32<u64> = to_float32_from_u64;

pub fn to_float32_from_u128(source: u128) -> Result<f32, ConversionFailure> {
    if is_u128_representable_in_f32(source) {
        Ok(source as f32)
    } else {
        Err(ConversionFailure::NotExactlyRepresentable)
    }
}
pub const TO_FLOAT32_FROM_U128: ToFloat32<u128> = to_float32_from_u128;

pub fn to_float32_from_f32(source: f32) -> Result<f32, ConversionFailure> {
    Ok(source)
}
pub const TO_FLOAT32_FROM_F32: ToFloat32<f32> = to_float32_from_f32;

pub fn to_float32_from_f64(source: f64) -> Result<f32, ConversionFailure> {
    f64_to_f32(source)
}
pub const TO_FLOAT32_FROM_F64: ToFloat32<f64> = to_float32_from_f64;
