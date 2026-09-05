use crate::definitions::failures::ConversionFailure;
use crate::definitions::value::{DynamicValue, OwnedDynamicValue};

pub type ToUint8<Source> = fn(Source) -> Result<u8, ConversionFailure>;

pub type ToUint8FromDynamic =
    for<'value> fn(&DynamicValue<'value>) -> Result<u8, ConversionFailure>;

pub type ToUint8FromOwnedDynamic = fn(&OwnedDynamicValue) -> Result<u8, ConversionFailure>;
