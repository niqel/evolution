use crate::definitions::failures::NumericFailure;
use crate::definitions::numeric::add::{Add, FloatAdd};

macro_rules! impl_add {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(lhs: $t, rhs: $t) -> Result<$t, NumericFailure> {
            lhs.checked_add(rhs).ok_or(NumericFailure::Overflow)
        }

        pub const $const_name: Add<$t> = $fn_name;
    };
}

impl_add!(add_i8, ADD_I8, i8);
impl_add!(add_i16, ADD_I16, i16);
impl_add!(add_i32, ADD_I32, i32);
impl_add!(add_i64, ADD_I64, i64);
impl_add!(add_i128, ADD_I128, i128);

impl_add!(add_u8, ADD_U8, u8);
impl_add!(add_u16, ADD_U16, u16);
impl_add!(add_u32, ADD_U32, u32);
impl_add!(add_u64, ADD_U64, u64);
impl_add!(add_u128, ADD_U128, u128);

macro_rules! impl_float_add {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(lhs: $t, rhs: $t) -> $t {
            lhs + rhs
        }

        pub const $const_name: FloatAdd<$t> = $fn_name;
    };
}

impl_float_add!(add_f32, ADD_F32, f32);
impl_float_add!(add_f64, ADD_F64, f64);
