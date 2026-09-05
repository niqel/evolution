use crate::definitions::failures::NumericFailure;

pub type Abs<T> = fn(T) -> Result<T, NumericFailure>;
pub type FloatAbs<T> = fn(T) -> T;
