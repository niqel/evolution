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
