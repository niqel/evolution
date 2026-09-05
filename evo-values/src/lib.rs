#![no_std]

extern crate alloc;

pub mod bitwise;
pub mod boolean;
pub mod definitions;
pub mod materialize_owned;
pub mod numeric;
pub mod text;

pub use definitions::control::ProductionControl;
pub use definitions::failures::{
    BitwiseFailure, ComparisonFailure, ConversionFailure, NumericFailure, TextOperationFailure,
};
pub use definitions::materialize_owned::MaterializeOwned;
pub use definitions::scalars::{PowerExponent, ShiftAmount, TextLength, TextPosition};
pub use definitions::value::{
    DynamicIntegerValue, DynamicValue, EnumPayload, OwnedDynamicInteger, OwnedDynamicValue,
    OwnedEnumPayload, OwnedValue, Value,
};
pub use materialize_owned::{MATERIALIZE_OWNED, materialize_owned};
