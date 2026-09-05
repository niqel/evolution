#![no_std]

extern crate alloc;

pub mod definitions;
pub mod text;

pub use definitions::control::ProductionControl;
pub use definitions::failures::{
    BitwiseFailure, ComparisonFailure, ConversionFailure, NumericFailure, TextOperationFailure,
};
pub use definitions::scalars::{PowerExponent, ShiftAmount, TextLength, TextPosition};
pub use definitions::value::{
    DynamicIntegerValue, DynamicValue, EnumPayload, OwnedDynamicInteger, OwnedDynamicValue,
    OwnedEnumPayload, OwnedValue, Value,
};
