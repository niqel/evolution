use crate::definitions::failures::NumericFailure;

pub type Abs<T> = fn(T) -> Result<T, NumericFailure>;
