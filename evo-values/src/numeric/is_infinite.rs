use crate::definitions::numeric::is_infinite::FloatIsInfinite;

pub fn is_infinite_f32(val: f32) -> bool {
    val.is_infinite()
}

pub const IS_INFINITE_F32: FloatIsInfinite<f32> = is_infinite_f32;

pub fn is_infinite_f64(val: f64) -> bool {
    val.is_infinite()
}

pub const IS_INFINITE_F64: FloatIsInfinite<f64> = is_infinite_f64;
