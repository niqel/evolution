use crate::conversion::kernel::{f32_to_f64, is_u128_representable_in_f64};
use crate::definitions::conversion::to_float64::ToFloat64;
use crate::definitions::failures::ConversionFailure;

pub fn to_float64_from_i8(source: i8) -> Result<f64, ConversionFailure> {
    Ok(source as f64)
}
pub const TO_FLOAT64_FROM_I8: ToFloat64<i8> = to_float64_from_i8;

pub fn to_float64_from_i16(source: i16) -> Result<f64, ConversionFailure> {
    Ok(source as f64)
}
pub const TO_FLOAT64_FROM_I16: ToFloat64<i16> = to_float64_from_i16;

pub fn to_float64_from_i32(source: i32) -> Result<f64, ConversionFailure> {
    Ok(source as f64)
}
pub const TO_FLOAT64_FROM_I32: ToFloat64<i32> = to_float64_from_i32;

pub fn to_float64_from_i64(source: i64) -> Result<f64, ConversionFailure> {
    if is_u128_representable_in_f64(source.unsigned_abs() as u128) {
        Ok(source as f64)
    } else {
        Err(ConversionFailure::NotExactlyRepresentable)
    }
}
pub const TO_FLOAT64_FROM_I64: ToFloat64<i64> = to_float64_from_i64;

pub fn to_float64_from_i128(source: i128) -> Result<f64, ConversionFailure> {
    if is_u128_representable_in_f64(source.unsigned_abs()) {
        Ok(source as f64)
    } else {
        Err(ConversionFailure::NotExactlyRepresentable)
    }
}
pub const TO_FLOAT64_FROM_I128: ToFloat64<i128> = to_float64_from_i128;

pub fn to_float64_from_u8(source: u8) -> Result<f64, ConversionFailure> {
    Ok(source as f64)
}
pub const TO_FLOAT64_FROM_U8: ToFloat64<u8> = to_float64_from_u8;

pub fn to_float64_from_u16(source: u16) -> Result<f64, ConversionFailure> {
    Ok(source as f64)
}
pub const TO_FLOAT64_FROM_U16: ToFloat64<u16> = to_float64_from_u16;

pub fn to_float64_from_u32(source: u32) -> Result<f64, ConversionFailure> {
    Ok(source as f64)
}
pub const TO_FLOAT64_FROM_U32: ToFloat64<u32> = to_float64_from_u32;

pub fn to_float64_from_u64(source: u64) -> Result<f64, ConversionFailure> {
    if is_u128_representable_in_f64(source as u128) {
        Ok(source as f64)
    } else {
        Err(ConversionFailure::NotExactlyRepresentable)
    }
}
pub const TO_FLOAT64_FROM_U64: ToFloat64<u64> = to_float64_from_u64;

pub fn to_float64_from_u128(source: u128) -> Result<f64, ConversionFailure> {
    if is_u128_representable_in_f64(source) {
        Ok(source as f64)
    } else {
        Err(ConversionFailure::NotExactlyRepresentable)
    }
}
pub const TO_FLOAT64_FROM_U128: ToFloat64<u128> = to_float64_from_u128;

pub fn to_float64_from_f32(source: f32) -> Result<f64, ConversionFailure> {
    f32_to_f64(source)
}
pub const TO_FLOAT64_FROM_F32: ToFloat64<f32> = to_float64_from_f32;

pub fn to_float64_from_f64(source: f64) -> Result<f64, ConversionFailure> {
    Ok(source)
}
pub const TO_FLOAT64_FROM_F64: ToFloat64<f64> = to_float64_from_f64;
