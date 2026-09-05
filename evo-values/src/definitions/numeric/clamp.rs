use crate::definitions::failures::NumericFailure;

pub type IntegerClamp<T> = fn(T, T, T) -> Result<T, NumericFailure>;
pub type FloatClamp<T> = fn(T, T, T) -> Result<T, NumericFailure>;
