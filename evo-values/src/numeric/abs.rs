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
