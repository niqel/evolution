use crate::definitions::bitwise::shift_left::ShiftLeft;
use crate::definitions::failures::BitwiseFailure;
use crate::definitions::scalars::ShiftAmount;

macro_rules! impl_shift_left {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(value: $t, shift: ShiftAmount) -> Result<$t, BitwiseFailure> {
            if shift.0 >= <$t>::BITS {
                return Err(BitwiseFailure::InvalidShift);
            }
            Ok(value << shift.0)
        }

        pub const $const_name: ShiftLeft<$t> = $fn_name;
    };
}

impl_shift_left!(shift_left_i8, SHIFT_LEFT_I8, i8);
impl_shift_left!(shift_left_i16, SHIFT_LEFT_I16, i16);
impl_shift_left!(shift_left_i32, SHIFT_LEFT_I32, i32);
impl_shift_left!(shift_left_i64, SHIFT_LEFT_I64, i64);
impl_shift_left!(shift_left_i128, SHIFT_LEFT_I128, i128);

impl_shift_left!(shift_left_u8, SHIFT_LEFT_U8, u8);
impl_shift_left!(shift_left_u16, SHIFT_LEFT_U16, u16);
impl_shift_left!(shift_left_u32, SHIFT_LEFT_U32, u32);
impl_shift_left!(shift_left_u64, SHIFT_LEFT_U64, u64);
impl_shift_left!(shift_left_u128, SHIFT_LEFT_U128, u128);
