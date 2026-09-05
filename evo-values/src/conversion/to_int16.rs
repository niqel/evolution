use crate::conversion::kernel::{decompose_f32_to_integer, decompose_f64_to_integer, fit_i16};
use crate::definitions::conversion::to_int16::ToInt16;
use crate::definitions::failures::ConversionFailure;

pub fn to_int16_from_i8(source: i8) -> Result<i16, ConversionFailure> {
    Ok(source as i16)
}
pub const TO_INT16_FROM_I8: ToInt16<i8> = to_int16_from_i8;

pub fn to_int16_from_i16(source: i16) -> Result<i16, ConversionFailure> {
    Ok(source)
}
pub const TO_INT16_FROM_I16: ToInt16<i16> = to_int16_from_i16;

pub fn to_int16_from_i32(source: i32) -> Result<i16, ConversionFailure> {
    i16::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_INT16_FROM_I32: ToInt16<i32> = to_int16_from_i32;

pub fn to_int16_from_i64(source: i64) -> Result<i16, ConversionFailure> {
    i16::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_INT16_FROM_I64: ToInt16<i64> = to_int16_from_i64;

pub fn to_int16_from_i128(source: i128) -> Result<i16, ConversionFailure> {
    i16::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_INT16_FROM_I128: ToInt16<i128> = to_int16_from_i128;

pub fn to_int16_from_u8(source: u8) -> Result<i16, ConversionFailure> {
    Ok(source as i16)
}
pub const TO_INT16_FROM_U8: ToInt16<u8> = to_int16_from_u8;

pub fn to_int16_from_u16(source: u16) -> Result<i16, ConversionFailure> {
    i16::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_INT16_FROM_U16: ToInt16<u16> = to_int16_from_u16;

pub fn to_int16_from_u32(source: u32) -> Result<i16, ConversionFailure> {
    i16::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_INT16_FROM_U32: ToInt16<u32> = to_int16_from_u32;

pub fn to_int16_from_u64(source: u64) -> Result<i16, ConversionFailure> {
    i16::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_INT16_FROM_U64: ToInt16<u64> = to_int16_from_u64;

pub fn to_int16_from_u128(source: u128) -> Result<i16, ConversionFailure> {
    i16::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_INT16_FROM_U128: ToInt16<u128> = to_int16_from_u128;

pub fn to_int16_from_f32(source: f32) -> Result<i16, ConversionFailure> {
    match decompose_f32_to_integer(source) {
        Some((sign, mag)) => fit_i16(sign, mag),
        None => Err(ConversionFailure::NotExactlyRepresentable),
    }
}
pub const TO_INT16_FROM_F32: ToInt16<f32> = to_int16_from_f32;

pub fn to_int16_from_f64(source: f64) -> Result<i16, ConversionFailure> {
    match decompose_f64_to_integer(source) {
        Some((sign, mag)) => fit_i16(sign, mag),
        None => Err(ConversionFailure::NotExactlyRepresentable),
    }
}
pub const TO_INT16_FROM_F64: ToInt16<f64> = to_int16_from_f64;

use crate::conversion::kernel::dynamic_integer_to_i16;
use crate::definitions::conversion::to_int16::{ToInt16FromDynamic, ToInt16FromOwnedDynamic};
use crate::definitions::value::{DynamicValue, OwnedDynamicValue};

pub fn to_int16_from_dynamic(source: &DynamicValue<'_>) -> Result<i16, ConversionFailure> {
    match source {
        DynamicValue::Integer(val) => dynamic_integer_to_i16(val.negative(), val.magnitude()),
        DynamicValue::Float32(val) => to_int16_from_f32(*val),
        DynamicValue::Float64(val) => to_int16_from_f64(*val),
    }
}
pub const TO_INT16_FROM_DYNAMIC: ToInt16FromDynamic = to_int16_from_dynamic;

pub fn to_int16_from_owned_dynamic(source: &OwnedDynamicValue) -> Result<i16, ConversionFailure> {
    match source {
        OwnedDynamicValue::Integer(val) => dynamic_integer_to_i16(val.negative(), val.magnitude()),
        OwnedDynamicValue::Float32(val) => to_int16_from_f32(*val),
        OwnedDynamicValue::Float64(val) => to_int16_from_f64(*val),
    }
}
pub const TO_INT16_FROM_OWNED_DYNAMIC: ToInt16FromOwnedDynamic = to_int16_from_owned_dynamic;
