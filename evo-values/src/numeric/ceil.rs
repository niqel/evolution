use crate::definitions::numeric::ceil::FloatCeil;

pub fn ceil_f32(val: f32) -> f32 {
    libm::ceilf(val)
}

pub const CEIL_F32: FloatCeil<f32> = ceil_f32;

pub fn ceil_f64(val: f64) -> f64 {
    libm::ceil(val)
}

pub const CEIL_F64: FloatCeil<f64> = ceil_f64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ceil_cases() {
        assert_eq!(ceil_f32(3.2), 4.0);
        assert_eq!(ceil_f32(-3.2), -3.0);
        assert_eq!(ceil_f32(3.0), 3.0);

        assert_eq!(ceil_f64(3.2), 4.0);
        assert_eq!(ceil_f64(-3.2), -3.0);
        assert_eq!(ceil_f64(3.0), 3.0);
    }

    #[test]
    fn ceil_constants() {
        let op: FloatCeil<f32> = CEIL_F32;
        assert_eq!(op(2.1), 3.0);

        let op64: FloatCeil<f64> = CEIL_F64;
        assert_eq!(op64(2.1), 3.0);
    }
}
