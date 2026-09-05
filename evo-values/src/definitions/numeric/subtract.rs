use crate::definitions::failures::NumericFailure;

pub type Subtract<T> = fn(T, T) -> Result<T, NumericFailure>;
