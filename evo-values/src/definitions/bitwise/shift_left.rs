use crate::definitions::failures::BitwiseFailure;
use crate::definitions::scalars::ShiftAmount;

pub type ShiftLeft<T> = fn(T, ShiftAmount) -> Result<T, BitwiseFailure>;
