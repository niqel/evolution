use crate::definitions::numeric::round::FloatRound;

pub fn round_f32(val: f32) -> f32 {
    libm::roundf(val)
}

pub const ROUND_F32: FloatRound<f32> = round_f32;

pub fn round_f64(val: f64) -> f64 {
    libm::round(val)
}

pub const ROUND_F64: FloatRound<f64> = round_f64;
