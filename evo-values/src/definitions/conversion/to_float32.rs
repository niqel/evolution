use crate::definitions::failures::ConversionFailure;
use crate::definitions::value::{DynamicValue, OwnedDynamicValue};

pub type ToFloat32<Source> = fn(Source) -> Result<f32, ConversionFailure>;

pub type ToFloat32FromDynamic =
    for<'value> fn(&DynamicValue<'value>) -> Result<f32, ConversionFailure>;

pub type ToFloat32FromOwnedDynamic = fn(&OwnedDynamicValue) -> Result<f32, ConversionFailure>;
