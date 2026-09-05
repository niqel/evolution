use crate::definitions::failures::ConversionFailure;
use crate::definitions::value::{DynamicValue, OwnedDynamicValue};

pub type ToInt8<Source> = fn(Source) -> Result<i8, ConversionFailure>;

pub type ToInt8FromDynamic = for<'value> fn(&DynamicValue<'value>) -> Result<i8, ConversionFailure>;

pub type ToInt8FromOwnedDynamic = fn(&OwnedDynamicValue) -> Result<i8, ConversionFailure>;
