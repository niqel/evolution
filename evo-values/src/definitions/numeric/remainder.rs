use crate::definitions::failures::NumericFailure;

pub type Remainder<T> = fn(T, T) -> Result<T, NumericFailure>;
