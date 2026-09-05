use crate::definitions::failures::ConversionFailure;

pub type ToUint8<Source> = fn(Source) -> Result<u8, ConversionFailure>;
