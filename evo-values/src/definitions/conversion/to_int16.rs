use crate::definitions::failures::ConversionFailure;

pub type ToInt16<Source> = fn(Source) -> Result<i16, ConversionFailure>;
