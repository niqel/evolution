use crate::definitions::failures::ConversionFailure;

pub type ToInt128<Source> = fn(Source) -> Result<i128, ConversionFailure>;
