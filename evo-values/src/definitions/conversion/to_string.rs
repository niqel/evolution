use crate::definitions::value::{
    DynamicIntegerValue, DynamicValue, OwnedDynamicInteger, OwnedDynamicValue,
};
use alloc::string::String;

pub type BooleanToString = fn(bool) -> &'static str;

pub type StringToString = for<'text> fn(&'text str) -> &'text str;

pub type IntegerToString<T> = fn(T) -> String;

pub type FloatToString<T> = fn(T) -> String;

pub type DynamicIntegerToString = for<'value> fn(&DynamicIntegerValue<'value>) -> String;

pub type OwnedDynamicIntegerToString = fn(&OwnedDynamicInteger) -> String;

pub type DynamicToString = for<'value> fn(&DynamicValue<'value>) -> String;

pub type OwnedDynamicToString = fn(&OwnedDynamicValue) -> String;
