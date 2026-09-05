use crate::definitions::failures::ConversionFailure;

pub type ToUint64<Source> = fn(Source) -> Result<u64, ConversionFailure>;
