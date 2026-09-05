use crate::definitions::failures::NumericFailure;

pub type Negate<T> = fn(T) -> Result<T, NumericFailure>;
