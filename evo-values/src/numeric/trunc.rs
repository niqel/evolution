use crate::definitions::numeric::trunc::FloatTrunc;

pub fn trunc_f32(val: f32) -> f32 {
    libm::truncf(val)
}

pub const TRUNC_F32: FloatTrunc<f32> = trunc_f32;

pub fn trunc_f64(val: f64) -> f64 {
    libm::trunc(val)
}

pub const TRUNC_F64: FloatTrunc<f64> = trunc_f64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trunc_cases() {
        assert_eq!(trunc_f32(3.7), 3.0);
        assert_eq!(trunc_f32(-3.7), -3.0);
        assert_eq!(trunc_f32(0.5), 0.0);

        assert_eq!(trunc_f64(3.7), 3.0);
        assert_eq!(trunc_f64(-3.7), -3.0);
        assert_eq!(trunc_f64(0.5), 0.0);
    }

    #[test]
    fn trunc_constants() {
        let op: FloatTrunc<f32> = TRUNC_F32;
        assert_eq!(op(2.9), 2.0);

        let op64: FloatTrunc<f64> = TRUNC_F64;
        assert_eq!(op64(2.9), 2.0);
    }
}
