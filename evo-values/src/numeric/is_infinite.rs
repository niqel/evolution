use crate::definitions::numeric::is_infinite::FloatIsInfinite;

pub fn is_infinite_f32(val: f32) -> bool {
    val.is_infinite()
}

pub const IS_INFINITE_F32: FloatIsInfinite<f32> = is_infinite_f32;

pub fn is_infinite_f64(val: f64) -> bool {
    val.is_infinite()
}

pub const IS_INFINITE_F64: FloatIsInfinite<f64> = is_infinite_f64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_infinite_cases() {
        assert!(is_infinite_f32(f32::INFINITY));
        assert!(is_infinite_f32(f32::NEG_INFINITY));
        assert!(!is_infinite_f32(0.0));
        assert!(!is_infinite_f32(f32::NAN));

        assert!(is_infinite_f64(f64::INFINITY));
        assert!(is_infinite_f64(f64::NEG_INFINITY));
        assert!(!is_infinite_f64(0.0));
        assert!(!is_infinite_f64(f64::NAN));
    }

    #[test]
    fn is_infinite_constants() {
        let op: FloatIsInfinite<f32> = IS_INFINITE_F32;
        assert!(op(f32::INFINITY));
        assert!(!op(1.0));

        let op64: FloatIsInfinite<f64> = IS_INFINITE_F64;
        assert!(op64(f64::NEG_INFINITY));
        assert!(!op64(1.0));
    }
}
