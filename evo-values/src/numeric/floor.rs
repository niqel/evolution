use crate::definitions::numeric::floor::FloatFloor;

pub fn floor_f32(val: f32) -> f32 {
    libm::floorf(val)
}

pub const FLOOR_F32: FloatFloor<f32> = floor_f32;

pub fn floor_f64(val: f64) -> f64 {
    libm::floor(val)
}

pub const FLOOR_F64: FloatFloor<f64> = floor_f64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floor_cases() {
        assert_eq!(floor_f32(3.7), 3.0);
        assert_eq!(floor_f32(-3.7), -4.0);
        assert_eq!(floor_f32(3.0), 3.0);

        assert_eq!(floor_f64(3.7), 3.0);
        assert_eq!(floor_f64(-3.7), -4.0);
        assert_eq!(floor_f64(3.0), 3.0);
    }

    #[test]
    fn floor_constants() {
        let op: FloatFloor<f32> = FLOOR_F32;
        assert_eq!(op(2.9), 2.0);

        let op64: FloatFloor<f64> = FLOOR_F64;
        assert_eq!(op64(2.9), 2.0);
    }
}
