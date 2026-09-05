use crate::definitions::failures::NumericFailure;

pub type IntegerClamp<T> = fn(T, T, T) -> Result<T, NumericFailure>;
