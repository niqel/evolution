use crate::definitions::numeric::round::FloatRound;

pub fn round_f32(val: f32) -> f32 {
    libm::roundf(val)
}

pub const ROUND_F32: FloatRound<f32> = round_f32;

pub fn round_f64(val: f64) -> f64 {
    libm::round(val)
}

pub const ROUND_F64: FloatRound<f64> = round_f64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_cases() {
        assert_eq!(round_f32(3.2), 3.0);
        assert_eq!(round_f32(3.5), 4.0);
        assert_eq!(round_f32(3.7), 4.0);
        assert_eq!(round_f32(-3.5), -4.0);

        assert_eq!(round_f64(3.2), 3.0);
        assert_eq!(round_f64(3.5), 4.0);
        assert_eq!(round_f64(3.7), 4.0);
        assert_eq!(round_f64(-3.5), -4.0);
    }

    #[test]
    fn round_constants() {
        let op: FloatRound<f32> = ROUND_F32;
        assert_eq!(op(2.5), 3.0);

        let op64: FloatRound<f64> = ROUND_F64;
        assert_eq!(op64(2.5), 3.0);
    }
}
