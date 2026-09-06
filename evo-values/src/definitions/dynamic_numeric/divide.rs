use crate::definitions::failures::DynamicNumericFailure;
use crate::definitions::value::{DynamicValue, OwnedDynamicValue};

pub type DynamicDivide = for<'left, 'right> fn(
    &DynamicValue<'left>,
    &DynamicValue<'right>,
) -> Result<OwnedDynamicValue, DynamicNumericFailure>;
