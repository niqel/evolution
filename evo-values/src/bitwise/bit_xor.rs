use crate::definitions::bitwise::bit_xor::BitXor;

macro_rules! impl_bit_xor {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(lhs: $t, rhs: $t) -> $t {
            lhs ^ rhs
        }

        pub const $const_name: BitXor<$t> = $fn_name;
    };
}

impl_bit_xor!(bit_xor_i8, BIT_XOR_I8, i8);
impl_bit_xor!(bit_xor_i16, BIT_XOR_I16, i16);
impl_bit_xor!(bit_xor_i32, BIT_XOR_I32, i32);
impl_bit_xor!(bit_xor_i64, BIT_XOR_I64, i64);
impl_bit_xor!(bit_xor_i128, BIT_XOR_I128, i128);

impl_bit_xor!(bit_xor_u8, BIT_XOR_U8, u8);
impl_bit_xor!(bit_xor_u16, BIT_XOR_U16, u16);
impl_bit_xor!(bit_xor_u32, BIT_XOR_U32, u32);
impl_bit_xor!(bit_xor_u64, BIT_XOR_U64, u64);
impl_bit_xor!(bit_xor_u128, BIT_XOR_U128, u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_xor_basic_cases() {
        assert_eq!(bit_xor_u8(0b1100, 0b1010), 0b0110);
        assert_eq!(bit_xor_i32(0b1100, 0b1010), 0b0110);
        assert_eq!(bit_xor_i32(42, 42), 0);
        assert_eq!(bit_xor_i32(42, 0), 42);
    }

    #[test]
    fn bit_xor_constants() {
        let op_signed: BitXor<i32> = BIT_XOR_I32;
        assert_eq!(op_signed(0xFF, 0x0F), 0xF0);

        let op_unsigned: BitXor<u64> = BIT_XOR_U64;
        assert_eq!(op_unsigned(0xFF, 0x0F), 0xF0);
    }
}
