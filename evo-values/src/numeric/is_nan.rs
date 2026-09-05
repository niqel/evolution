use crate::definitions::numeric::is_nan::FloatIsNan;

pub fn is_nan_f32(val: f32) -> bool {
    val.is_nan()
}

pub const IS_NAN_F32: FloatIsNan<f32> = is_nan_f32;

pub fn is_nan_f64(val: f64) -> bool {
    val.is_nan()
}

pub const IS_NAN_F64: FloatIsNan<f64> = is_nan_f64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_nan_cases() {
        assert!(is_nan_f32(f32::NAN));
        assert!(!is_nan_f32(0.0));
        assert!(!is_nan_f32(f32::INFINITY));

        assert!(is_nan_f64(f64::NAN));
        assert!(!is_nan_f64(0.0));
        assert!(!is_nan_f64(f64::INFINITY));
    }

    #[test]
    fn is_nan_constants() {
        let op: FloatIsNan<f32> = IS_NAN_F32;
        assert!(op(f32::NAN));
        assert!(!op(1.0));

        let op64: FloatIsNan<f64> = IS_NAN_F64;
        assert!(op64(f64::NAN));
        assert!(!op64(1.0));
    }
}
