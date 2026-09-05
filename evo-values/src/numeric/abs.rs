use crate::definitions::failures::NumericFailure;
use crate::definitions::numeric::abs::{Abs, FloatAbs};

macro_rules! impl_abs {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(val: $t) -> Result<$t, NumericFailure> {
            val.checked_abs().ok_or(NumericFailure::Overflow)
        }

        pub const $const_name: Abs<$t> = $fn_name;
    };
}

impl_abs!(abs_i8, ABS_I8, i8);
impl_abs!(abs_i16, ABS_I16, i16);
impl_abs!(abs_i32, ABS_I32, i32);
impl_abs!(abs_i64, ABS_I64, i64);
impl_abs!(abs_i128, ABS_I128, i128);

macro_rules! impl_float_abs {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(val: $t) -> $t {
            val.abs()
        }

        pub const $const_name: FloatAbs<$t> = $fn_name;
    };
}

impl_float_abs!(abs_f32, ABS_F32, f32);
impl_float_abs!(abs_f64, ABS_F64, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abs_signed_cases() {
        assert_eq!(abs_i8(-5), Ok(5));
        assert_eq!(abs_i8(5), Ok(5));
        assert_eq!(abs_i8(0), Ok(0));
        assert_eq!(abs_i8(i8::MIN), Err(NumericFailure::Overflow));

        assert_eq!(abs_i16(i16::MIN), Err(NumericFailure::Overflow));
        assert_eq!(abs_i32(i32::MIN), Err(NumericFailure::Overflow));
        assert_eq!(abs_i64(i64::MIN), Err(NumericFailure::Overflow));
        assert_eq!(abs_i128(i128::MIN), Err(NumericFailure::Overflow));
    }

    #[test]
    fn abs_float_cases() {
        assert_eq!(abs_f32(-5.5), 5.5);
        assert_eq!(abs_f32(5.5), 5.5);
        assert_eq!(abs_f32(-0.0), 0.0);
        assert_eq!(abs_f64(-5.5), 5.5);
        assert_eq!(abs_f64(5.5), 5.5);
        assert_eq!(abs_f64(-0.0), 0.0);
    }

    #[test]
    fn abs_constants() {
        let op: Abs<i32> = ABS_I32;
        assert_eq!(op(-42), Ok(42));

        let op_float: FloatAbs<f32> = ABS_F32;
        assert_eq!(op_float(-42.0), 42.0);
    }
}
