use crate::definitions::failures::ConversionFailure;

pub type ToInt32<Source> = fn(Source) -> Result<i32, ConversionFailure>;
