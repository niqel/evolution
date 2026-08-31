#![no_std]

extern crate alloc;

pub mod definitions;
pub mod text;

pub use definitions::value::{
    DynamicIntegerValue, DynamicValue, EnumPayload, OwnedDynamicInteger, OwnedDynamicValue,
    OwnedEnumPayload, OwnedValue, Value,
};
