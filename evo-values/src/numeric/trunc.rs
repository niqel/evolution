use crate::definitions::numeric::trunc::FloatTrunc;

pub fn trunc_f32(val: f32) -> f32 {
    libm::truncf(val)
}

pub const TRUNC_F32: FloatTrunc<f32> = trunc_f32;

pub fn trunc_f64(val: f64) -> f64 {
    libm::trunc(val)
}

pub const TRUNC_F64: FloatTrunc<f64> = trunc_f64;
