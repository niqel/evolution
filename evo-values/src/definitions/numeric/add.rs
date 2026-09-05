use crate::definitions::failures::NumericFailure;

pub type Add<T> = fn(T, T) -> Result<T, NumericFailure>;
pub type FloatAdd<T> = fn(T, T) -> T;
