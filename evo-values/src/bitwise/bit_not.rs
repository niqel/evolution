use crate::definitions::bitwise::bit_not::BitNot;

macro_rules! impl_bit_not {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(value: $t) -> $t {
            !value
        }

        pub const $const_name: BitNot<$t> = $fn_name;
    };
}

impl_bit_not!(bit_not_i8, BIT_NOT_I8, i8);
impl_bit_not!(bit_not_i16, BIT_NOT_I16, i16);
impl_bit_not!(bit_not_i32, BIT_NOT_I32, i32);
impl_bit_not!(bit_not_i64, BIT_NOT_I64, i64);
impl_bit_not!(bit_not_i128, BIT_NOT_I128, i128);

impl_bit_not!(bit_not_u8, BIT_NOT_U8, u8);
impl_bit_not!(bit_not_u16, BIT_NOT_U16, u16);
impl_bit_not!(bit_not_u32, BIT_NOT_U32, u32);
impl_bit_not!(bit_not_u64, BIT_NOT_U64, u64);
impl_bit_not!(bit_not_u128, BIT_NOT_U128, u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_not_basic_cases() {
        assert_eq!(bit_not_u8(0), 255);
        assert_eq!(bit_not_u8(255), 0);
        assert_eq!(bit_not_i8(0), -1);
        assert_eq!(bit_not_i8(-1), 0);
    }

    #[test]
    fn bit_not_constants() {
        let op_signed: BitNot<i32> = BIT_NOT_I32;
        assert_eq!(op_signed(0), -1);

        let op_unsigned: BitNot<u64> = BIT_NOT_U64;
        assert_eq!(op_unsigned(0), u64::MAX);
    }
}
