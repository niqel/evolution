use crate::definitions::numeric::max::{FloatMax, IntegerMax};

macro_rules! impl_max {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(a: $t, b: $t) -> $t {
            if a >= b { a } else { b }
        }

        pub const $const_name: IntegerMax<$t> = $fn_name;
    };
}

impl_max!(max_i8, MAX_I8, i8);
impl_max!(max_i16, MAX_I16, i16);
impl_max!(max_i32, MAX_I32, i32);
impl_max!(max_i64, MAX_I64, i64);
impl_max!(max_i128, MAX_I128, i128);

impl_max!(max_u8, MAX_U8, u8);
impl_max!(max_u16, MAX_U16, u16);
impl_max!(max_u32, MAX_U32, u32);
impl_max!(max_u64, MAX_U64, u64);
impl_max!(max_u128, MAX_U128, u128);

macro_rules! impl_float_max {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(a: $t, b: $t) -> $t {
            a.max(b)
        }

        pub const $const_name: FloatMax<$t> = $fn_name;
    };
}

impl_float_max!(max_f32, MAX_F32, f32);
impl_float_max!(max_f64, MAX_F64, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_signed() {
        assert_eq!(max_i32(2, 5), 5);
        assert_eq!(max_i32(5, 2), 5);
        assert_eq!(max_i32(5, 5), 5);
        assert_eq!(max_i32(i32::MIN, i32::MAX), i32::MAX);
    }

    #[test]
    fn max_unsigned() {
        assert_eq!(max_u32(2, 5), 5);
        assert_eq!(max_u32(5, 2), 5);
        assert_eq!(max_u32(5, 5), 5);
        assert_eq!(max_u32(0, u32::MAX), u32::MAX);
    }

    #[test]
    fn max_float() {
        assert_eq!(max_f32(2.0, 5.0), 5.0);
        assert_eq!(max_f32(5.0, 2.0), 5.0);
        assert_eq!(max_f64(2.0, 5.0), 5.0);
        assert_eq!(max_f64(5.0, 2.0), 5.0);
    }

    #[test]
    fn max_constants() {
        let op: IntegerMax<u64> = MAX_U64;
        assert_eq!(op(10, 20), 20);

        let op_float: FloatMax<f64> = MAX_F64;
        assert_eq!(op_float(10.0, 20.0), 20.0);
    }
}
