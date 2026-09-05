use crate::definitions::failures::NumericFailure;
use crate::definitions::numeric::pow::Pow;
use crate::definitions::scalars::PowerExponent;

macro_rules! impl_pow {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(base: $t, exponent: PowerExponent) -> Result<$t, NumericFailure> {
            base.checked_pow(exponent.0).ok_or(NumericFailure::Overflow)
        }

        pub const $const_name: Pow<$t> = $fn_name;
    };
}

impl_pow!(pow_i8, POW_I8, i8);
impl_pow!(pow_i16, POW_I16, i16);
impl_pow!(pow_i32, POW_I32, i32);
impl_pow!(pow_i64, POW_I64, i64);
impl_pow!(pow_i128, POW_I128, i128);

impl_pow!(pow_u8, POW_U8, u8);
impl_pow!(pow_u16, POW_U16, u16);
impl_pow!(pow_u32, POW_U32, u32);
impl_pow!(pow_u64, POW_U64, u64);
impl_pow!(pow_u128, POW_U128, u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pow_signed_cases() {
        assert_eq!(pow_i8(2, PowerExponent(3)), Ok(8));
        assert_eq!(pow_i8(5, PowerExponent(0)), Ok(1));
        assert_eq!(pow_i8(0, PowerExponent(0)), Ok(1));
        assert_eq!(pow_i8(5, PowerExponent(1)), Ok(5));
        assert_eq!(pow_i8(-2, PowerExponent(3)), Ok(-8));
        assert_eq!(pow_i8(-2, PowerExponent(2)), Ok(4));
        assert_eq!(pow_i8(2, PowerExponent(7)), Err(NumericFailure::Overflow));
    }

    #[test]
    fn pow_unsigned_cases() {
        assert_eq!(pow_u8(2, PowerExponent(3)), Ok(8));
        assert_eq!(pow_u8(5, PowerExponent(0)), Ok(1));
        assert_eq!(pow_u8(0, PowerExponent(0)), Ok(1));
        assert_eq!(pow_u8(5, PowerExponent(1)), Ok(5));
        assert_eq!(pow_u8(2, PowerExponent(8)), Err(NumericFailure::Overflow));
    }

    #[test]
    fn pow_constants() {
        let op_signed: Pow<i32> = POW_I32;
        assert_eq!(op_signed(2, PowerExponent(10)), Ok(1024));

        let op_unsigned: Pow<u64> = POW_U64;
        assert_eq!(op_unsigned(2, PowerExponent(10)), Ok(1024));
    }
}
