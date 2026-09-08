use crate::definitions::numeric::min::{FloatMin, IntegerMin};

macro_rules! impl_min {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(a: $t, b: $t) -> $t {
            if a <= b { a } else { b }
        }

        pub const $const_name: IntegerMin<$t> = $fn_name;
    };
}

impl_min!(min_i8, MIN_I8, i8);
impl_min!(min_i16, MIN_I16, i16);
impl_min!(min_i32, MIN_I32, i32);
impl_min!(min_i64, MIN_I64, i64);
impl_min!(min_i128, MIN_I128, i128);

impl_min!(min_u8, MIN_U8, u8);
impl_min!(min_u16, MIN_U16, u16);
impl_min!(min_u32, MIN_U32, u32);
impl_min!(min_u64, MIN_U64, u64);
impl_min!(min_u128, MIN_U128, u128);

macro_rules! impl_float_min {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(a: $t, b: $t) -> $t {
            a.min(b)
        }

        pub const $const_name: FloatMin<$t> = $fn_name;
    };
}

impl_float_min!(min_f32, MIN_F32, f32);
impl_float_min!(min_f64, MIN_F64, f64);
