use crate::definitions::numeric::floor::FloatFloor;

pub fn floor_f32(val: f32) -> f32 {
    libm::floorf(val)
}

pub const FLOOR_F32: FloatFloor<f32> = floor_f32;

pub fn floor_f64(val: f64) -> f64 {
    libm::floor(val)
}

pub const FLOOR_F64: FloatFloor<f64> = floor_f64;
