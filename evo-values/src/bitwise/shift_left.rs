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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_left_basic_cases() {
        assert_eq!(shift_left_u8(1, ShiftAmount(0)), Ok(1));
        assert_eq!(shift_left_u8(1, ShiftAmount(1)), Ok(2));
        assert_eq!(shift_left_u8(1, ShiftAmount(7)), Ok(128));
        assert_eq!(
            shift_left_u8(1, ShiftAmount(8)),
            Err(BitwiseFailure::InvalidShift)
        );
        assert_eq!(
            shift_left_u8(1, ShiftAmount(9)),
            Err(BitwiseFailure::InvalidShift)
        );
        assert_eq!(shift_left_u8(0b1000_0001, ShiftAmount(1)), Ok(0b0000_0010));
    }

    #[test]
    fn shift_left_constants() {
        let op_signed: ShiftLeft<i32> = SHIFT_LEFT_I32;
        assert_eq!(op_signed(1, ShiftAmount(4)), Ok(16));

        let op_unsigned: ShiftLeft<u64> = SHIFT_LEFT_U64;
        assert_eq!(op_unsigned(1, ShiftAmount(4)), Ok(16));
    }
}
