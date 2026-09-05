use crate::conversion::kernel::{decompose_f32_to_integer, decompose_f64_to_integer, fit_i32};
use crate::definitions::conversion::to_int32::ToInt32;
use crate::definitions::failures::ConversionFailure;

pub fn to_int32_from_i8(source: i8) -> Result<i32, ConversionFailure> {
    Ok(source as i32)
}
pub const TO_INT32_FROM_I8: ToInt32<i8> = to_int32_from_i8;

pub fn to_int32_from_i16(source: i16) -> Result<i32, ConversionFailure> {
    Ok(source as i32)
}
pub const TO_INT32_FROM_I16: ToInt32<i16> = to_int32_from_i16;

pub fn to_int32_from_i32(source: i32) -> Result<i32, ConversionFailure> {
    Ok(source)
}
pub const TO_INT32_FROM_I32: ToInt32<i32> = to_int32_from_i32;

pub fn to_int32_from_i64(source: i64) -> Result<i32, ConversionFailure> {
    i32::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_INT32_FROM_I64: ToInt32<i64> = to_int32_from_i64;

pub fn to_int32_from_i128(source: i128) -> Result<i32, ConversionFailure> {
    i32::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_INT32_FROM_I128: ToInt32<i128> = to_int32_from_i128;

pub fn to_int32_from_u8(source: u8) -> Result<i32, ConversionFailure> {
    Ok(source as i32)
}
pub const TO_INT32_FROM_U8: ToInt32<u8> = to_int32_from_u8;

pub fn to_int32_from_u16(source: u16) -> Result<i32, ConversionFailure> {
    Ok(source as i32)
}
pub const TO_INT32_FROM_U16: ToInt32<u16> = to_int32_from_u16;

pub fn to_int32_from_u32(source: u32) -> Result<i32, ConversionFailure> {
    i32::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_INT32_FROM_U32: ToInt32<u32> = to_int32_from_u32;

pub fn to_int32_from_u64(source: u64) -> Result<i32, ConversionFailure> {
    i32::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_INT32_FROM_U64: ToInt32<u64> = to_int32_from_u64;

pub fn to_int32_from_u128(source: u128) -> Result<i32, ConversionFailure> {
    i32::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_INT32_FROM_U128: ToInt32<u128> = to_int32_from_u128;

pub fn to_int32_from_f32(source: f32) -> Result<i32, ConversionFailure> {
    match decompose_f32_to_integer(source) {
        Some((sign, mag)) => fit_i32(sign, mag),
        None => Err(ConversionFailure::NotExactlyRepresentable),
    }
}
pub const TO_INT32_FROM_F32: ToInt32<f32> = to_int32_from_f32;

pub fn to_int32_from_f64(source: f64) -> Result<i32, ConversionFailure> {
    match decompose_f64_to_integer(source) {
        Some((sign, mag)) => fit_i32(sign, mag),
        None => Err(ConversionFailure::NotExactlyRepresentable),
    }
}
pub const TO_INT32_FROM_F64: ToInt32<f64> = to_int32_from_f64;

use crate::conversion::kernel::dynamic_integer_to_i32;
use crate::definitions::conversion::to_int32::{ToInt32FromDynamic, ToInt32FromOwnedDynamic};
use crate::definitions::value::{DynamicValue, OwnedDynamicValue};

pub fn to_int32_from_dynamic(source: &DynamicValue<'_>) -> Result<i32, ConversionFailure> {
    match source {
        DynamicValue::Integer(val) => dynamic_integer_to_i32(val.negative(), val.magnitude()),
        DynamicValue::Float32(val) => to_int32_from_f32(*val),
        DynamicValue::Float64(val) => to_int32_from_f64(*val),
    }
}
pub const TO_INT32_FROM_DYNAMIC: ToInt32FromDynamic = to_int32_from_dynamic;

pub fn to_int32_from_owned_dynamic(source: &OwnedDynamicValue) -> Result<i32, ConversionFailure> {
    match source {
        OwnedDynamicValue::Integer(val) => dynamic_integer_to_i32(val.negative(), val.magnitude()),
        OwnedDynamicValue::Float32(val) => to_int32_from_f32(*val),
        OwnedDynamicValue::Float64(val) => to_int32_from_f64(*val),
    }
}
pub const TO_INT32_FROM_OWNED_DYNAMIC: ToInt32FromOwnedDynamic = to_int32_from_owned_dynamic;
