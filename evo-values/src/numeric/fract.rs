use crate::definitions::numeric::fract::FloatFract;

pub fn fract_f32(val: f32) -> f32 {
    val - libm::truncf(val)
}

pub const FRACT_F32: FloatFract<f32> = fract_f32;

pub fn fract_f64(val: f64) -> f64 {
    val - libm::trunc(val)
}

pub const FRACT_F64: FloatFract<f64> = fract_f64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fract_cases() {
        assert!((fract_f32(3.5) - 0.5).abs() < 1e-6);
        assert!((fract_f32(-3.5) - -0.5).abs() < 1e-6);
        assert_eq!(fract_f32(3.0), 0.0);

        assert!((fract_f64(3.5) - 0.5).abs() < 1e-10);
        assert!((fract_f64(-3.5) - -0.5).abs() < 1e-10);
        assert_eq!(fract_f64(3.0), 0.0);
    }

    #[test]
    fn fract_constants() {
        let op: FloatFract<f32> = FRACT_F32;
        assert!((op(1.25) - 0.25).abs() < 1e-6);

        let op64: FloatFract<f64> = FRACT_F64;
        assert!((op64(1.25) - 0.25).abs() < 1e-10);
    }
}
