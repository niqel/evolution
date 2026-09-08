use crate::definitions::numeric::ceil::FloatCeil;

pub fn ceil_f32(val: f32) -> f32 {
    libm::ceilf(val)
}

pub const CEIL_F32: FloatCeil<f32> = ceil_f32;

pub fn ceil_f64(val: f64) -> f64 {
    libm::ceil(val)
}

pub const CEIL_F64: FloatCeil<f64> = ceil_f64;
