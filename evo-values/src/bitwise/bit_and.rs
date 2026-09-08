use crate::definitions::bitwise::bit_and::BitAnd;

macro_rules! impl_bit_and {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(lhs: $t, rhs: $t) -> $t {
            lhs & rhs
        }

        pub const $const_name: BitAnd<$t> = $fn_name;
    };
}

impl_bit_and!(bit_and_i8, BIT_AND_I8, i8);
impl_bit_and!(bit_and_i16, BIT_AND_I16, i16);
impl_bit_and!(bit_and_i32, BIT_AND_I32, i32);
impl_bit_and!(bit_and_i64, BIT_AND_I64, i64);
impl_bit_and!(bit_and_i128, BIT_AND_I128, i128);

impl_bit_and!(bit_and_u8, BIT_AND_U8, u8);
impl_bit_and!(bit_and_u16, BIT_AND_U16, u16);
impl_bit_and!(bit_and_u32, BIT_AND_U32, u32);
impl_bit_and!(bit_and_u64, BIT_AND_U64, u64);
impl_bit_and!(bit_and_u128, BIT_AND_U128, u128);
