use crate::definitions::failures::ConversionFailure;
use crate::definitions::value::{DynamicValue, OwnedDynamicValue};

pub type ToUint16<Source> = fn(Source) -> Result<u16, ConversionFailure>;

pub type ToUint16FromDynamic =
    for<'value> fn(&DynamicValue<'value>) -> Result<u16, ConversionFailure>;

pub type ToUint16FromOwnedDynamic = fn(&OwnedDynamicValue) -> Result<u16, ConversionFailure>;
