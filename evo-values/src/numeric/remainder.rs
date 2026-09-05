use crate::definitions::failures::NumericFailure;
use crate::definitions::numeric::remainder::{FloatRemainder, Remainder};

macro_rules! impl_remainder {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(lhs: $t, rhs: $t) -> Result<$t, NumericFailure> {
            if rhs == 0 {
                return Err(NumericFailure::DivisionByZero);
            }
            lhs.checked_rem(rhs).ok_or(NumericFailure::Overflow)
        }

        pub const $const_name: Remainder<$t> = $fn_name;
    };
}

impl_remainder!(remainder_i8, REMAINDER_I8, i8);
impl_remainder!(remainder_i16, REMAINDER_I16, i16);
impl_remainder!(remainder_i32, REMAINDER_I32, i32);
impl_remainder!(remainder_i64, REMAINDER_I64, i64);
impl_remainder!(remainder_i128, REMAINDER_I128, i128);

impl_remainder!(remainder_u8, REMAINDER_U8, u8);
impl_remainder!(remainder_u16, REMAINDER_U16, u16);
impl_remainder!(remainder_u32, REMAINDER_U32, u32);
impl_remainder!(remainder_u64, REMAINDER_U64, u64);
impl_remainder!(remainder_u128, REMAINDER_U128, u128);

macro_rules! impl_float_remainder {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(lhs: $t, rhs: $t) -> $t {
            lhs % rhs
        }

        pub const $const_name: FloatRemainder<$t> = $fn_name;
    };
}

impl_float_remainder!(remainder_f32, REMAINDER_F32, f32);
impl_float_remainder!(remainder_f64, REMAINDER_F64, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remainder_signed_cases() {
        assert_eq!(remainder_i8(7, 3), Ok(1));
        assert_eq!(remainder_i8(-7, 3), Ok(-1));
        assert_eq!(remainder_i8(7, -3), Ok(1));
        assert_eq!(remainder_i8(-7, -3), Ok(-1));
        assert_eq!(remainder_i8(42, 0), Err(NumericFailure::DivisionByZero));
        assert_eq!(remainder_i8(i8::MIN, -1), Err(NumericFailure::Overflow));

        assert_eq!(remainder_i16(i16::MIN, -1), Err(NumericFailure::Overflow));
        assert_eq!(remainder_i32(i32::MIN, -1), Err(NumericFailure::Overflow));
        assert_eq!(remainder_i64(i64::MIN, -1), Err(NumericFailure::Overflow));
        assert_eq!(remainder_i128(i128::MIN, -1), Err(NumericFailure::Overflow));
    }

    #[test]
    fn remainder_unsigned_cases() {
        assert_eq!(remainder_u8(7, 3), Ok(1));
        assert_eq!(remainder_u8(42, 0), Err(NumericFailure::DivisionByZero));
    }

    #[test]
    fn remainder_float_cases() {
        assert_eq!(remainder_f32(7.5, 2.5), 0.0);
        assert_eq!(remainder_f32(7.0, 2.5), 2.0);
        assert!(remainder_f32(7.0, 0.0).is_nan());

        assert_eq!(remainder_f64(7.5, 2.5), 0.0);
        assert_eq!(remainder_f64(7.0, 2.5), 2.0);
        assert!(remainder_f64(7.0, 0.0).is_nan());
    }

    #[test]
    fn remainder_constants() {
        let op_signed: Remainder<i32> = REMAINDER_I32;
        assert_eq!(op_signed(10, 3), Ok(1));

        let op_unsigned: Remainder<u64> = REMAINDER_U64;
        assert_eq!(op_unsigned(10, 3), Ok(1));

        let op_float: FloatRemainder<f64> = REMAINDER_F64;
        assert_eq!(op_float(7.0, 2.5), 2.0);
    }
}
