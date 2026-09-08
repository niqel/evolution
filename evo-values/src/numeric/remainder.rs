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
