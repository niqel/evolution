use crate::definitions::failures::NumericFailure;

pub type Negate<T> = fn(T) -> Result<T, NumericFailure>;
pub type FloatNegate<T> = fn(T) -> T;
