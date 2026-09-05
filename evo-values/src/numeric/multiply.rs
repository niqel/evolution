use crate::definitions::failures::NumericFailure;
use crate::definitions::numeric::multiply::{FloatMultiply, Multiply};

macro_rules! impl_multiply {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(lhs: $t, rhs: $t) -> Result<$t, NumericFailure> {
            lhs.checked_mul(rhs).ok_or(NumericFailure::Overflow)
        }

        pub const $const_name: Multiply<$t> = $fn_name;
    };
}

impl_multiply!(multiply_i8, MULTIPLY_I8, i8);
impl_multiply!(multiply_i16, MULTIPLY_I16, i16);
impl_multiply!(multiply_i32, MULTIPLY_I32, i32);
impl_multiply!(multiply_i64, MULTIPLY_I64, i64);
impl_multiply!(multiply_i128, MULTIPLY_I128, i128);

impl_multiply!(multiply_u8, MULTIPLY_U8, u8);
impl_multiply!(multiply_u16, MULTIPLY_U16, u16);
impl_multiply!(multiply_u32, MULTIPLY_U32, u32);
impl_multiply!(multiply_u64, MULTIPLY_U64, u64);
impl_multiply!(multiply_u128, MULTIPLY_U128, u128);

macro_rules! impl_float_multiply {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(lhs: $t, rhs: $t) -> $t {
            lhs * rhs
        }

        pub const $const_name: FloatMultiply<$t> = $fn_name;
    };
}

impl_float_multiply!(multiply_f32, MULTIPLY_F32, f32);
impl_float_multiply!(multiply_f64, MULTIPLY_F64, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiply_signed_cases() {
        assert_eq!(multiply_i8(6, 7), Ok(42));
        assert_eq!(multiply_i8(0, 100), Ok(0));
        assert_eq!(multiply_i8(1, 42), Ok(42));
        assert_eq!(multiply_i8(-1, 42), Ok(-42));
        assert_eq!(multiply_i8(i8::MAX, 2), Err(NumericFailure::Overflow));
        assert_eq!(multiply_i8(i8::MIN, -1), Err(NumericFailure::Overflow));
    }

    #[test]
    fn multiply_unsigned_cases() {
        assert_eq!(multiply_u8(6, 7), Ok(42));
        assert_eq!(multiply_u8(0, 100), Ok(0));
        assert_eq!(multiply_u8(1, 42), Ok(42));
        assert_eq!(multiply_u8(u8::MAX, 2), Err(NumericFailure::Overflow));
    }

    #[test]
    fn multiply_float_cases() {
        assert_eq!(multiply_f32(2.5, 4.0), 10.0);
        assert_eq!(multiply_f64(2.5, 4.0), 10.0);
    }

    #[test]
    fn multiply_constants() {
        let op_signed: Multiply<i32> = MULTIPLY_I32;
        assert_eq!(op_signed(5, 6), Ok(30));

        let op_unsigned: Multiply<u64> = MULTIPLY_U64;
        assert_eq!(op_unsigned(5, 6), Ok(30));

        let op_float: FloatMultiply<f32> = MULTIPLY_F32;
        assert_eq!(op_float(5.0, 6.0), 30.0);
    }
}
