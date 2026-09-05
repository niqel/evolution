use crate::definitions::conversion::to_dynamic_integer::ToDynamicInteger;
use crate::definitions::value::{OwnedDynamicInteger, OwnedDynamicValue};
use alloc::boxed::Box;

pub fn to_dynamic_integer_from_i8(source: i8) -> OwnedDynamicValue {
    OwnedDynamicValue::Integer(OwnedDynamicInteger::from_parts(
        source < 0,
        Box::from(source.unsigned_abs().to_be_bytes().as_slice()),
    ))
}
pub const TO_DYNAMIC_INTEGER_FROM_I8: ToDynamicInteger<i8> = to_dynamic_integer_from_i8;

pub fn to_dynamic_integer_from_i16(source: i16) -> OwnedDynamicValue {
    OwnedDynamicValue::Integer(OwnedDynamicInteger::from_parts(
        source < 0,
        Box::from(source.unsigned_abs().to_be_bytes().as_slice()),
    ))
}
pub const TO_DYNAMIC_INTEGER_FROM_I16: ToDynamicInteger<i16> = to_dynamic_integer_from_i16;

pub fn to_dynamic_integer_from_i32(source: i32) -> OwnedDynamicValue {
    OwnedDynamicValue::Integer(OwnedDynamicInteger::from_parts(
        source < 0,
        Box::from(source.unsigned_abs().to_be_bytes().as_slice()),
    ))
}
pub const TO_DYNAMIC_INTEGER_FROM_I32: ToDynamicInteger<i32> = to_dynamic_integer_from_i32;

pub fn to_dynamic_integer_from_i64(source: i64) -> OwnedDynamicValue {
    OwnedDynamicValue::Integer(OwnedDynamicInteger::from_parts(
        source < 0,
        Box::from(source.unsigned_abs().to_be_bytes().as_slice()),
    ))
}
pub const TO_DYNAMIC_INTEGER_FROM_I64: ToDynamicInteger<i64> = to_dynamic_integer_from_i64;

pub fn to_dynamic_integer_from_i128(source: i128) -> OwnedDynamicValue {
    OwnedDynamicValue::Integer(OwnedDynamicInteger::from_parts(
        source < 0,
        Box::from(source.unsigned_abs().to_be_bytes().as_slice()),
    ))
}
pub const TO_DYNAMIC_INTEGER_FROM_I128: ToDynamicInteger<i128> = to_dynamic_integer_from_i128;

pub fn to_dynamic_integer_from_u8(source: u8) -> OwnedDynamicValue {
    OwnedDynamicValue::Integer(OwnedDynamicInteger::from_parts(
        false,
        Box::from(source.to_be_bytes().as_slice()),
    ))
}
pub const TO_DYNAMIC_INTEGER_FROM_U8: ToDynamicInteger<u8> = to_dynamic_integer_from_u8;

pub fn to_dynamic_integer_from_u16(source: u16) -> OwnedDynamicValue {
    OwnedDynamicValue::Integer(OwnedDynamicInteger::from_parts(
        false,
        Box::from(source.to_be_bytes().as_slice()),
    ))
}
pub const TO_DYNAMIC_INTEGER_FROM_U16: ToDynamicInteger<u16> = to_dynamic_integer_from_u16;

pub fn to_dynamic_integer_from_u32(source: u32) -> OwnedDynamicValue {
    OwnedDynamicValue::Integer(OwnedDynamicInteger::from_parts(
        false,
        Box::from(source.to_be_bytes().as_slice()),
    ))
}
pub const TO_DYNAMIC_INTEGER_FROM_U32: ToDynamicInteger<u32> = to_dynamic_integer_from_u32;

pub fn to_dynamic_integer_from_u64(source: u64) -> OwnedDynamicValue {
    OwnedDynamicValue::Integer(OwnedDynamicInteger::from_parts(
        false,
        Box::from(source.to_be_bytes().as_slice()),
    ))
}
pub const TO_DYNAMIC_INTEGER_FROM_U64: ToDynamicInteger<u64> = to_dynamic_integer_from_u64;

pub fn to_dynamic_integer_from_u128(source: u128) -> OwnedDynamicValue {
    OwnedDynamicValue::Integer(OwnedDynamicInteger::from_parts(
        false,
        Box::from(source.to_be_bytes().as_slice()),
    ))
}
pub const TO_DYNAMIC_INTEGER_FROM_U128: ToDynamicInteger<u128> = to_dynamic_integer_from_u128;
