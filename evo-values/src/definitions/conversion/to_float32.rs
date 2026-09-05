use crate::definitions::failures::ConversionFailure;

pub type ToFloat32<Source> = fn(Source) -> Result<f32, ConversionFailure>;
