use crate::conversion::kernel::dynamic_integer_to_decimal;
use crate::definitions::conversion::to_string::{
    BooleanToString, DynamicIntegerToString, DynamicToString, FloatToString, IntegerToString,
    OwnedDynamicIntegerToString, OwnedDynamicToString, StringToString,
};
use crate::definitions::value::{
    DynamicIntegerValue, DynamicValue, OwnedDynamicInteger, OwnedDynamicValue,
};
use alloc::format;
use alloc::string::String;

// ============================================================================
// 1. Boolean
// ============================================================================

pub fn boolean_to_string(source: bool) -> &'static str {
    if source { "true" } else { "false" }
}
pub const BOOLEAN_TO_STRING: BooleanToString = boolean_to_string;

// ============================================================================
// 2. String Identity
// ============================================================================

pub fn string_to_string<'text>(source: &'text str) -> &'text str {
    source
}
pub const STRING_TO_STRING: StringToString = string_to_string;

// ============================================================================
// 3. Fixed Signed Integers
// ============================================================================

pub fn to_string_from_i8(source: i8) -> String {
    format!("{}", source)
}
pub const TO_STRING_FROM_I8: IntegerToString<i8> = to_string_from_i8;

pub fn to_string_from_i16(source: i16) -> String {
    format!("{}", source)
}
pub const TO_STRING_FROM_I16: IntegerToString<i16> = to_string_from_i16;

pub fn to_string_from_i32(source: i32) -> String {
    format!("{}", source)
}
pub const TO_STRING_FROM_I32: IntegerToString<i32> = to_string_from_i32;

pub fn to_string_from_i64(source: i64) -> String {
    format!("{}", source)
}
pub const TO_STRING_FROM_I64: IntegerToString<i64> = to_string_from_i64;

pub fn to_string_from_i128(source: i128) -> String {
    format!("{}", source)
}
pub const TO_STRING_FROM_I128: IntegerToString<i128> = to_string_from_i128;

// ============================================================================
// 4. Fixed Unsigned Integers
// ============================================================================

pub fn to_string_from_u8(source: u8) -> String {
    format!("{}", source)
}
pub const TO_STRING_FROM_U8: IntegerToString<u8> = to_string_from_u8;

pub fn to_string_from_u16(source: u16) -> String {
    format!("{}", source)
}
pub const TO_STRING_FROM_U16: IntegerToString<u16> = to_string_from_u16;

pub fn to_string_from_u32(source: u32) -> String {
    format!("{}", source)
}
pub const TO_STRING_FROM_U32: IntegerToString<u32> = to_string_from_u32;

pub fn to_string_from_u64(source: u64) -> String {
    format!("{}", source)
}
pub const TO_STRING_FROM_U64: IntegerToString<u64> = to_string_from_u64;

pub fn to_string_from_u128(source: u128) -> String {
    format!("{}", source)
}
pub const TO_STRING_FROM_U128: IntegerToString<u128> = to_string_from_u128;

// ============================================================================
// 5. Fixed Floats
// ============================================================================

pub fn to_string_from_f32(source: f32) -> String {
    format!("{}", source)
}
pub const TO_STRING_FROM_F32: FloatToString<f32> = to_string_from_f32;

pub fn to_string_from_f64(source: f64) -> String {
    format!("{}", source)
}
pub const TO_STRING_FROM_F64: FloatToString<f64> = to_string_from_f64;

// ============================================================================
// 6. Dynamic Integer
// ============================================================================

pub fn to_string_from_dynamic_integer(source: &DynamicIntegerValue<'_>) -> String {
    dynamic_integer_to_decimal(source.negative(), source.magnitude())
}
pub const TO_STRING_FROM_DYNAMIC_INTEGER: DynamicIntegerToString = to_string_from_dynamic_integer;

pub fn to_string_from_owned_dynamic_integer(source: &OwnedDynamicInteger) -> String {
    dynamic_integer_to_decimal(source.negative(), source.magnitude())
}
pub const TO_STRING_FROM_OWNED_DYNAMIC_INTEGER: OwnedDynamicIntegerToString =
    to_string_from_owned_dynamic_integer;

// ============================================================================
// 7. Dynamic Value Dispatch
// ============================================================================

pub fn to_string_from_dynamic(source: &DynamicValue<'_>) -> String {
    match source {
        DynamicValue::Integer(val) => to_string_from_dynamic_integer(val),
        DynamicValue::Float32(val) => to_string_from_f32(*val),
        DynamicValue::Float64(val) => to_string_from_f64(*val),
    }
}
pub const TO_STRING_FROM_DYNAMIC: DynamicToString = to_string_from_dynamic;

pub fn to_string_from_owned_dynamic(source: &OwnedDynamicValue) -> String {
    match source {
        OwnedDynamicValue::Integer(val) => to_string_from_owned_dynamic_integer(val),
        OwnedDynamicValue::Float32(val) => to_string_from_f32(*val),
        OwnedDynamicValue::Float64(val) => to_string_from_f64(*val),
    }
}
pub const TO_STRING_FROM_OWNED_DYNAMIC: OwnedDynamicToString = to_string_from_owned_dynamic;
