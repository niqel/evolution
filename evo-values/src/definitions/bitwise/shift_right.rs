use crate::definitions::failures::BitwiseFailure;
use crate::definitions::scalars::ShiftAmount;

pub type ShiftRight<T> = fn(T, ShiftAmount) -> Result<T, BitwiseFailure>;
