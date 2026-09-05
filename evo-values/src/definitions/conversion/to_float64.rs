use crate::definitions::failures::ConversionFailure;

pub type ToFloat64<Source> = fn(Source) -> Result<f64, ConversionFailure>;
