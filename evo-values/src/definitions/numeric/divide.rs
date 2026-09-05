use crate::definitions::failures::NumericFailure;

pub type Divide<T> = fn(T, T) -> Result<T, NumericFailure>;
