use crate::definitions::numeric::is_finite::FloatIsFinite;

pub fn is_finite_f32(val: f32) -> bool {
    val.is_finite()
}

pub const IS_FINITE_F32: FloatIsFinite<f32> = is_finite_f32;

pub fn is_finite_f64(val: f64) -> bool {
    val.is_finite()
}

pub const IS_FINITE_F64: FloatIsFinite<f64> = is_finite_f64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_finite_cases() {
        assert!(is_finite_f32(0.0));
        assert!(is_finite_f32(-100.5));
        assert!(!is_finite_f32(f32::INFINITY));
        assert!(!is_finite_f32(f32::NEG_INFINITY));
        assert!(!is_finite_f32(f32::NAN));

        assert!(is_finite_f64(0.0));
        assert!(is_finite_f64(-100.5));
        assert!(!is_finite_f64(f64::INFINITY));
        assert!(!is_finite_f64(f64::NEG_INFINITY));
        assert!(!is_finite_f64(f64::NAN));
    }

    #[test]
    fn is_finite_constants() {
        let op: FloatIsFinite<f32> = IS_FINITE_F32;
        assert!(op(1.0));
        assert!(!op(f32::INFINITY));

        let op64: FloatIsFinite<f64> = IS_FINITE_F64;
        assert!(op64(1.0));
        assert!(!op64(f64::NAN));
    }
}
