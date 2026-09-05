use crate::definitions::failures::NumericFailure;

pub type Multiply<T> = fn(T, T) -> Result<T, NumericFailure>;
