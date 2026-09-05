use crate::definitions::failures::ConversionFailure;
use crate::definitions::value::{DynamicValue, OwnedDynamicValue};

pub type ToInt64<Source> = fn(Source) -> Result<i64, ConversionFailure>;

pub type ToInt64FromDynamic =
    for<'value> fn(&DynamicValue<'value>) -> Result<i64, ConversionFailure>;

pub type ToInt64FromOwnedDynamic = fn(&OwnedDynamicValue) -> Result<i64, ConversionFailure>;
