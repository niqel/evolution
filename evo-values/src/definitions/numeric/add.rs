use crate::definitions::failures::NumericFailure;

pub type Add<T> = fn(T, T) -> Result<T, NumericFailure>;
