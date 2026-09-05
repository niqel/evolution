use crate::definitions::failures::ConversionFailure;

pub type ToUint16<Source> = fn(Source) -> Result<u16, ConversionFailure>;
