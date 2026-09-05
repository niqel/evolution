use crate::definitions::failures::NumericFailure;
use crate::definitions::numeric::subtract::Subtract;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtract_signed_cases() {
        assert_eq!(subtract_i8(50, 8), Ok(42));
        assert_eq!(subtract_i8(i8::MIN, 0), Ok(i8::MIN));
        assert_eq!(subtract_i8(i8::MIN, 1), Err(NumericFailure::Overflow));
    }

    #[test]
    fn subtract_unsigned_cases() {
        assert_eq!(subtract_u8(50, 8), Ok(42));
        assert_eq!(subtract_u8(0, 0), Ok(0));
        assert_eq!(subtract_u8(0, 1), Err(NumericFailure::Overflow));
    }

    #[test]
    fn subtract_constants() {
        let op_signed: Subtract<i32> = SUBTRACT_I32;
        assert_eq!(op_signed(30, 10), Ok(20));

        let op_unsigned: Subtract<u64> = SUBTRACT_U64;
        assert_eq!(op_unsigned(30, 10), Ok(20));
    }
}
