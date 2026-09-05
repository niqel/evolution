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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_signed_cases() {
        assert_eq!(add_i8(20, 22), Ok(42));
        assert_eq!(add_i8(i8::MAX, 0), Ok(i8::MAX));
        assert_eq!(add_i8(i8::MAX, 1), Err(NumericFailure::Overflow));
        assert_eq!(add_i8(i8::MIN, -1), Err(NumericFailure::Overflow));
    }

    #[test]
    fn add_unsigned_cases() {
        assert_eq!(add_u8(20, 22), Ok(42));
        assert_eq!(add_u8(u8::MAX, 0), Ok(u8::MAX));
        assert_eq!(add_u8(u8::MAX, 1), Err(NumericFailure::Overflow));
    }

    #[test]
    fn add_float_cases() {
        assert_eq!(add_f32(1.5, 2.5), 4.0);
        assert_eq!(add_f32(f32::MAX, f32::MAX), f32::INFINITY);
        assert_eq!(add_f64(1.5, 2.5), 4.0);
        assert_eq!(add_f64(f64::MAX, f64::MAX), f64::INFINITY);
    }

    #[test]
    fn add_constants() {
        let op_signed: Add<i32> = ADD_I32;
        assert_eq!(op_signed(10, 20), Ok(30));

        let op_unsigned: Add<u64> = ADD_U64;
        assert_eq!(op_unsigned(10, 20), Ok(30));

        let op_float: FloatAdd<f32> = ADD_F32;
        assert_eq!(op_float(10.0, 20.0), 30.0);
    }
}
