use crate::definitions::failures::NumericFailure;
use crate::definitions::numeric::subtract::{FloatSubtract, Subtract};

macro_rules! impl_subtract {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(lhs: $t, rhs: $t) -> Result<$t, NumericFailure> {
            lhs.checked_sub(rhs).ok_or(NumericFailure::Overflow)
        }

        pub const $const_name: Subtract<$t> = $fn_name;
    };
}

impl_subtract!(subtract_i8, SUBTRACT_I8, i8);
impl_subtract!(subtract_i16, SUBTRACT_I16, i16);
impl_subtract!(subtract_i32, SUBTRACT_I32, i32);
impl_subtract!(subtract_i64, SUBTRACT_I64, i64);
impl_subtract!(subtract_i128, SUBTRACT_I128, i128);

impl_subtract!(subtract_u8, SUBTRACT_U8, u8);
impl_subtract!(subtract_u16, SUBTRACT_U16, u16);
impl_subtract!(subtract_u32, SUBTRACT_U32, u32);
impl_subtract!(subtract_u64, SUBTRACT_U64, u64);
impl_subtract!(subtract_u128, SUBTRACT_U128, u128);

macro_rules! impl_float_subtract {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(lhs: $t, rhs: $t) -> $t {
            lhs - rhs
        }

        pub const $const_name: FloatSubtract<$t> = $fn_name;
    };
}

impl_float_subtract!(subtract_f32, SUBTRACT_F32, f32);
impl_float_subtract!(subtract_f64, SUBTRACT_F64, f64);
