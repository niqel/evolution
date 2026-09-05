use crate::definitions::failures::ConversionFailure;
use crate::definitions::value::{DynamicValue, OwnedDynamicValue};

pub type ToInt128<Source> = fn(Source) -> Result<i128, ConversionFailure>;

pub type ToInt128FromDynamic =
    for<'value> fn(&DynamicValue<'value>) -> Result<i128, ConversionFailure>;

pub type ToInt128FromOwnedDynamic = fn(&OwnedDynamicValue) -> Result<i128, ConversionFailure>;
