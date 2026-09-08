use crate::definitions::numeric::is_finite::FloatIsFinite;

pub fn is_finite_f32(val: f32) -> bool {
    val.is_finite()
}

pub const IS_FINITE_F32: FloatIsFinite<f32> = is_finite_f32;

pub fn is_finite_f64(val: f64) -> bool {
    val.is_finite()
}

pub const IS_FINITE_F64: FloatIsFinite<f64> = is_finite_f64;
