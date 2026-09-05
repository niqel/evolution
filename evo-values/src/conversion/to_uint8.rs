use crate::conversion::kernel::{decompose_f32_to_integer, decompose_f64_to_integer, fit_u8};
use crate::definitions::conversion::to_uint8::ToUint8;
use crate::definitions::failures::ConversionFailure;

pub fn to_uint8_from_i8(source: i8) -> Result<u8, ConversionFailure> {
    u8::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_UINT8_FROM_I8: ToUint8<i8> = to_uint8_from_i8;

pub fn to_uint8_from_i16(source: i16) -> Result<u8, ConversionFailure> {
    u8::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_UINT8_FROM_I16: ToUint8<i16> = to_uint8_from_i16;

pub fn to_uint8_from_i32(source: i32) -> Result<u8, ConversionFailure> {
    u8::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_UINT8_FROM_I32: ToUint8<i32> = to_uint8_from_i32;

pub fn to_uint8_from_i64(source: i64) -> Result<u8, ConversionFailure> {
    u8::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_UINT8_FROM_I64: ToUint8<i64> = to_uint8_from_i64;

pub fn to_uint8_from_i128(source: i128) -> Result<u8, ConversionFailure> {
    u8::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_UINT8_FROM_I128: ToUint8<i128> = to_uint8_from_i128;

pub fn to_uint8_from_u8(source: u8) -> Result<u8, ConversionFailure> {
    Ok(source)
}
pub const TO_UINT8_FROM_U8: ToUint8<u8> = to_uint8_from_u8;

pub fn to_uint8_from_u16(source: u16) -> Result<u8, ConversionFailure> {
    u8::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_UINT8_FROM_U16: ToUint8<u16> = to_uint8_from_u16;

pub fn to_uint8_from_u32(source: u32) -> Result<u8, ConversionFailure> {
    u8::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_UINT8_FROM_U32: ToUint8<u32> = to_uint8_from_u32;

pub fn to_uint8_from_u64(source: u64) -> Result<u8, ConversionFailure> {
    u8::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_UINT8_FROM_U64: ToUint8<u64> = to_uint8_from_u64;

pub fn to_uint8_from_u128(source: u128) -> Result<u8, ConversionFailure> {
    u8::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_UINT8_FROM_U128: ToUint8<u128> = to_uint8_from_u128;

pub fn to_uint8_from_f32(source: f32) -> Result<u8, ConversionFailure> {
    match decompose_f32_to_integer(source) {
        Some((sign, mag)) => fit_u8(sign, mag),
        None => Err(ConversionFailure::NotExactlyRepresentable),
    }
}
pub const TO_UINT8_FROM_F32: ToUint8<f32> = to_uint8_from_f32;

pub fn to_uint8_from_f64(source: f64) -> Result<u8, ConversionFailure> {
    match decompose_f64_to_integer(source) {
        Some((sign, mag)) => fit_u8(sign, mag),
        None => Err(ConversionFailure::NotExactlyRepresentable),
    }
}
pub const TO_UINT8_FROM_F64: ToUint8<f64> = to_uint8_from_f64;
