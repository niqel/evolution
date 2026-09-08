use crate::definitions::numeric::is_nan::FloatIsNan;

pub fn is_nan_f32(val: f32) -> bool {
    val.is_nan()
}

pub const IS_NAN_F32: FloatIsNan<f32> = is_nan_f32;

pub fn is_nan_f64(val: f64) -> bool {
    val.is_nan()
}

pub const IS_NAN_F64: FloatIsNan<f64> = is_nan_f64;
