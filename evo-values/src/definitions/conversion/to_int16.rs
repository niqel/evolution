use crate::definitions::failures::ConversionFailure;
use crate::definitions::value::{DynamicValue, OwnedDynamicValue};

pub type ToInt16<Source> = fn(Source) -> Result<i16, ConversionFailure>;

pub type ToInt16FromDynamic =
    for<'value> fn(&DynamicValue<'value>) -> Result<i16, ConversionFailure>;

pub type ToInt16FromOwnedDynamic = fn(&OwnedDynamicValue) -> Result<i16, ConversionFailure>;
