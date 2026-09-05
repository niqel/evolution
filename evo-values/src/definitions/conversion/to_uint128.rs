use crate::definitions::failures::ConversionFailure;

pub type ToUint128<Source> = fn(Source) -> Result<u128, ConversionFailure>;
