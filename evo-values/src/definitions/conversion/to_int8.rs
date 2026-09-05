use crate::definitions::failures::ConversionFailure;

pub type ToInt8<Source> = fn(Source) -> Result<i8, ConversionFailure>;
