use crate::definitions::failures::ConversionFailure;
use crate::definitions::value::{DynamicValue, OwnedDynamicValue};

pub type ToUint128<Source> = fn(Source) -> Result<u128, ConversionFailure>;

pub type ToUint128FromDynamic =
    for<'value> fn(&DynamicValue<'value>) -> Result<u128, ConversionFailure>;

pub type ToUint128FromOwnedDynamic = fn(&OwnedDynamicValue) -> Result<u128, ConversionFailure>;
