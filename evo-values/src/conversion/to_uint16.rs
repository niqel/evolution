use crate::conversion::kernel::{decompose_f32_to_integer, decompose_f64_to_integer, fit_u16};
use crate::definitions::conversion::to_uint16::ToUint16;
use crate::definitions::failures::ConversionFailure;

pub fn to_uint16_from_i8(source: i8) -> Result<u16, ConversionFailure> {
    u16::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_UINT16_FROM_I8: ToUint16<i8> = to_uint16_from_i8;

pub fn to_uint16_from_i16(source: i16) -> Result<u16, ConversionFailure> {
    u16::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_UINT16_FROM_I16: ToUint16<i16> = to_uint16_from_i16;

pub fn to_uint16_from_i32(source: i32) -> Result<u16, ConversionFailure> {
    u16::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_UINT16_FROM_I32: ToUint16<i32> = to_uint16_from_i32;

pub fn to_uint16_from_i64(source: i64) -> Result<u16, ConversionFailure> {
    u16::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_UINT16_FROM_I64: ToUint16<i64> = to_uint16_from_i64;

pub fn to_uint16_from_i128(source: i128) -> Result<u16, ConversionFailure> {
    u16::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_UINT16_FROM_I128: ToUint16<i128> = to_uint16_from_i128;

pub fn to_uint16_from_u8(source: u8) -> Result<u16, ConversionFailure> {
    Ok(source as u16)
}
pub const TO_UINT16_FROM_U8: ToUint16<u8> = to_uint16_from_u8;

pub fn to_uint16_from_u16(source: u16) -> Result<u16, ConversionFailure> {
    Ok(source)
}
pub const TO_UINT16_FROM_U16: ToUint16<u16> = to_uint16_from_u16;

pub fn to_uint16_from_u32(source: u32) -> Result<u16, ConversionFailure> {
    u16::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_UINT16_FROM_U32: ToUint16<u32> = to_uint16_from_u32;

pub fn to_uint16_from_u64(source: u64) -> Result<u16, ConversionFailure> {
    u16::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_UINT16_FROM_U64: ToUint16<u64> = to_uint16_from_u64;

pub fn to_uint16_from_u128(source: u128) -> Result<u16, ConversionFailure> {
    u16::try_from(source).map_err(|_| ConversionFailure::NotExactlyRepresentable)
}
pub const TO_UINT16_FROM_U128: ToUint16<u128> = to_uint16_from_u128;

pub fn to_uint16_from_f32(source: f32) -> Result<u16, ConversionFailure> {
    match decompose_f32_to_integer(source) {
        Some((sign, mag)) => fit_u16(sign, mag),
        None => Err(ConversionFailure::NotExactlyRepresentable),
    }
}
pub const TO_UINT16_FROM_F32: ToUint16<f32> = to_uint16_from_f32;

pub fn to_uint16_from_f64(source: f64) -> Result<u16, ConversionFailure> {
    match decompose_f64_to_integer(source) {
        Some((sign, mag)) => fit_u16(sign, mag),
        None => Err(ConversionFailure::NotExactlyRepresentable),
    }
}
pub const TO_UINT16_FROM_F64: ToUint16<f64> = to_uint16_from_f64;
