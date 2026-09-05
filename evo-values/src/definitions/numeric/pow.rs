use crate::definitions::failures::NumericFailure;
use crate::definitions::scalars::PowerExponent;

pub type Pow<T> = fn(T, PowerExponent) -> Result<T, NumericFailure>;
