use crate::definitions::failures::NumericFailure;

pub type Multiply<T> = fn(T, T) -> Result<T, NumericFailure>;
pub type FloatMultiply<T> = fn(T, T) -> T;
