use crate::definitions::failures::ConversionFailure;

pub type ToInt64<Source> = fn(Source) -> Result<i64, ConversionFailure>;
