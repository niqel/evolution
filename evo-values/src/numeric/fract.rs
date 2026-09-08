use crate::definitions::numeric::fract::FloatFract;

pub fn fract_f32(val: f32) -> f32 {
    val - libm::truncf(val)
}

pub const FRACT_F32: FloatFract<f32> = fract_f32;

pub fn fract_f64(val: f64) -> f64 {
    val - libm::trunc(val)
}

pub const FRACT_F64: FloatFract<f64> = fract_f64;
