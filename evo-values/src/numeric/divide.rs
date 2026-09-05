use crate::definitions::failures::NumericFailure;
use crate::definitions::numeric::divide::{Divide, FloatDivide};

macro_rules! impl_divide {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(lhs: $t, rhs: $t) -> Result<$t, NumericFailure> {
            if rhs == 0 {
                return Err(NumericFailure::DivisionByZero);
            }
            lhs.checked_div(rhs).ok_or(NumericFailure::Overflow)
        }

        pub const $const_name: Divide<$t> = $fn_name;
    };
}

impl_divide!(divide_i8, DIVIDE_I8, i8);
impl_divide!(divide_i16, DIVIDE_I16, i16);
impl_divide!(divide_i32, DIVIDE_I32, i32);
impl_divide!(divide_i64, DIVIDE_I64, i64);
impl_divide!(divide_i128, DIVIDE_I128, i128);

impl_divide!(divide_u8, DIVIDE_U8, u8);
impl_divide!(divide_u16, DIVIDE_U16, u16);
impl_divide!(divide_u32, DIVIDE_U32, u32);
impl_divide!(divide_u64, DIVIDE_U64, u64);
impl_divide!(divide_u128, DIVIDE_U128, u128);

macro_rules! impl_float_divide {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(lhs: $t, rhs: $t) -> $t {
            lhs / rhs
        }

        pub const $const_name: FloatDivide<$t> = $fn_name;
    };
}

impl_float_divide!(divide_f32, DIVIDE_F32, f32);
impl_float_divide!(divide_f64, DIVIDE_F64, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divide_signed_cases() {
        assert_eq!(divide_i8(84, 2), Ok(42));
        assert_eq!(divide_i8(42, 0), Err(NumericFailure::DivisionByZero));
        assert_eq!(divide_i8(-7, 3), Ok(-2));
        assert_eq!(divide_i8(7, -3), Ok(-2));
        assert_eq!(divide_i8(i8::MIN, -1), Err(NumericFailure::Overflow));

        assert_eq!(divide_i16(i16::MIN, -1), Err(NumericFailure::Overflow));
        assert_eq!(divide_i32(i32::MIN, -1), Err(NumericFailure::Overflow));
        assert_eq!(divide_i64(i64::MIN, -1), Err(NumericFailure::Overflow));
        assert_eq!(divide_i128(i128::MIN, -1), Err(NumericFailure::Overflow));
    }

    #[test]
    fn divide_unsigned_cases() {
        assert_eq!(divide_u8(84, 2), Ok(42));
        assert_eq!(divide_u8(42, 0), Err(NumericFailure::DivisionByZero));
    }

    #[test]
    fn divide_float_cases() {
        assert_eq!(divide_f32(84.0, 2.0), 42.0);
        assert_eq!(divide_f32(1.0, 0.0), f32::INFINITY);
        assert_eq!(divide_f32(-1.0, 0.0), f32::NEG_INFINITY);
        assert!(divide_f32(0.0, 0.0).is_nan());

        assert_eq!(divide_f64(84.0, 2.0), 42.0);
        assert_eq!(divide_f64(1.0, 0.0), f64::INFINITY);
        assert_eq!(divide_f64(-1.0, 0.0), f64::NEG_INFINITY);
        assert!(divide_f64(0.0, 0.0).is_nan());
    }

    #[test]
    fn divide_constants() {
        let op_signed: Divide<i32> = DIVIDE_I32;
        assert_eq!(op_signed(100, 5), Ok(20));

        let op_unsigned: Divide<u64> = DIVIDE_U64;
        assert_eq!(op_unsigned(100, 5), Ok(20));

        let op_float: FloatDivide<f64> = DIVIDE_F64;
        assert_eq!(op_float(100.0, 5.0), 20.0);
    }
}
