use crate::definitions::failures::ConversionFailure;
use crate::definitions::value::{DynamicValue, OwnedDynamicValue};

pub type ToUint64<Source> = fn(Source) -> Result<u64, ConversionFailure>;

pub type ToUint64FromDynamic =
    for<'value> fn(&DynamicValue<'value>) -> Result<u64, ConversionFailure>;

pub type ToUint64FromOwnedDynamic = fn(&OwnedDynamicValue) -> Result<u64, ConversionFailure>;
