use crate::definitions::failures::NumericFailure;

pub type Subtract<T> = fn(T, T) -> Result<T, NumericFailure>;
pub type FloatSubtract<T> = fn(T, T) -> T;
