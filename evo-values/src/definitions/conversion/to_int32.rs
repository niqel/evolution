use crate::definitions::failures::ConversionFailure;
use crate::definitions::value::{DynamicValue, OwnedDynamicValue};

pub type ToInt32<Source> = fn(Source) -> Result<i32, ConversionFailure>;

pub type ToInt32FromDynamic =
    for<'value> fn(&DynamicValue<'value>) -> Result<i32, ConversionFailure>;

pub type ToInt32FromOwnedDynamic = fn(&OwnedDynamicValue) -> Result<i32, ConversionFailure>;
