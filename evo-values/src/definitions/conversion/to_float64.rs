use crate::definitions::failures::ConversionFailure;
use crate::definitions::value::{DynamicValue, OwnedDynamicValue};

pub type ToFloat64<Source> = fn(Source) -> Result<f64, ConversionFailure>;

pub type ToFloat64FromDynamic =
    for<'value> fn(&DynamicValue<'value>) -> Result<f64, ConversionFailure>;

pub type ToFloat64FromOwnedDynamic = fn(&OwnedDynamicValue) -> Result<f64, ConversionFailure>;
