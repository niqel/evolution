use crate::definitions::failures::ConversionFailure;
use crate::definitions::value::{DynamicValue, OwnedDynamicValue};

pub type ToUint32<Source> = fn(Source) -> Result<u32, ConversionFailure>;

pub type ToUint32FromDynamic =
    for<'value> fn(&DynamicValue<'value>) -> Result<u32, ConversionFailure>;

pub type ToUint32FromOwnedDynamic = fn(&OwnedDynamicValue) -> Result<u32, ConversionFailure>;
