use crate::definitions::bitwise::bit_or::BitOr;

macro_rules! impl_bit_or {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(lhs: $t, rhs: $t) -> $t {
            lhs | rhs
        }

        pub const $const_name: BitOr<$t> = $fn_name;
    };
}

impl_bit_or!(bit_or_i8, BIT_OR_I8, i8);
impl_bit_or!(bit_or_i16, BIT_OR_I16, i16);
impl_bit_or!(bit_or_i32, BIT_OR_I32, i32);
impl_bit_or!(bit_or_i64, BIT_OR_I64, i64);
impl_bit_or!(bit_or_i128, BIT_OR_I128, i128);

impl_bit_or!(bit_or_u8, BIT_OR_U8, u8);
impl_bit_or!(bit_or_u16, BIT_OR_U16, u16);
impl_bit_or!(bit_or_u32, BIT_OR_U32, u32);
impl_bit_or!(bit_or_u64, BIT_OR_U64, u64);
impl_bit_or!(bit_or_u128, BIT_OR_U128, u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_or_basic_cases() {
        assert_eq!(bit_or_u8(0b1100, 0b1010), 0b1110);
        assert_eq!(bit_or_i32(0b1100, 0b1010), 0b1110);
        assert_eq!(bit_or_i8(0, -1), -1);
    }

    #[test]
    fn bit_or_constants() {
        let op_signed: BitOr<i32> = BIT_OR_I32;
        assert_eq!(op_signed(0xF0, 0x0F), 0xFF);

        let op_unsigned: BitOr<u64> = BIT_OR_U64;
        assert_eq!(op_unsigned(0xF0, 0x0F), 0xFF);
    }
}
