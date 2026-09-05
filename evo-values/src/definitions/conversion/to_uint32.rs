use crate::definitions::failures::ConversionFailure;

pub type ToUint32<Source> = fn(Source) -> Result<u32, ConversionFailure>;
