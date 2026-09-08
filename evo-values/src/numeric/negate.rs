use crate::definitions::failures::NumericFailure;
use crate::definitions::numeric::negate::{FloatNegate, Negate};

macro_rules! impl_negate {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(val: $t) -> Result<$t, NumericFailure> {
            val.checked_neg().ok_or(NumericFailure::Overflow)
        }

        pub const $const_name: Negate<$t> = $fn_name;
    };
}

impl_negate!(negate_i8, NEGATE_I8, i8);
impl_negate!(negate_i16, NEGATE_I16, i16);
impl_negate!(negate_i32, NEGATE_I32, i32);
impl_negate!(negate_i64, NEGATE_I64, i64);
impl_negate!(negate_i128, NEGATE_I128, i128);

macro_rules! impl_float_negate {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(val: $t) -> $t {
            -val
        }

        pub const $const_name: FloatNegate<$t> = $fn_name;
    };
}

impl_float_negate!(negate_f32, NEGATE_F32, f32);
impl_float_negate!(negate_f64, NEGATE_F64, f64);
