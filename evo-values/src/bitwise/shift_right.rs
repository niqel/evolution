use crate::definitions::bitwise::shift_right::ShiftRight;
use crate::definitions::failures::BitwiseFailure;
use crate::definitions::scalars::ShiftAmount;

macro_rules! impl_shift_right {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(value: $t, shift: ShiftAmount) -> Result<$t, BitwiseFailure> {
            if shift.0 >= <$t>::BITS {
                return Err(BitwiseFailure::InvalidShift);
            }
            Ok(value >> shift.0)
        }

        pub const $const_name: ShiftRight<$t> = $fn_name;
    };
}

impl_shift_right!(shift_right_i8, SHIFT_RIGHT_I8, i8);
impl_shift_right!(shift_right_i16, SHIFT_RIGHT_I16, i16);
impl_shift_right!(shift_right_i32, SHIFT_RIGHT_I32, i32);
impl_shift_right!(shift_right_i64, SHIFT_RIGHT_I64, i64);
impl_shift_right!(shift_right_i128, SHIFT_RIGHT_I128, i128);

impl_shift_right!(shift_right_u8, SHIFT_RIGHT_U8, u8);
impl_shift_right!(shift_right_u16, SHIFT_RIGHT_U16, u16);
impl_shift_right!(shift_right_u32, SHIFT_RIGHT_U32, u32);
impl_shift_right!(shift_right_u64, SHIFT_RIGHT_U64, u64);
impl_shift_right!(shift_right_u128, SHIFT_RIGHT_U128, u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_right_basic_cases() {
        assert_eq!(shift_right_u8(128, ShiftAmount(0)), Ok(128));
        assert_eq!(shift_right_u8(128, ShiftAmount(1)), Ok(64));
        assert_eq!(shift_right_u8(128, ShiftAmount(7)), Ok(1));
        assert_eq!(
            shift_right_u8(128, ShiftAmount(8)),
            Err(BitwiseFailure::InvalidShift)
        );
        assert_eq!(
            shift_right_u8(128, ShiftAmount(9)),
            Err(BitwiseFailure::InvalidShift)
        );
        // signed arithmetic shift right (sign extension)
        assert_eq!(shift_right_i8(-2, ShiftAmount(1)), Ok(-1));
        assert_eq!(shift_right_i8(-4, ShiftAmount(1)), Ok(-2));
        assert_eq!(shift_right_i8(-1, ShiftAmount(7)), Ok(-1));
    }

    #[test]
    fn shift_right_constants() {
        let op_signed: ShiftRight<i32> = SHIFT_RIGHT_I32;
        assert_eq!(op_signed(16, ShiftAmount(2)), Ok(4));

        let op_unsigned: ShiftRight<u64> = SHIFT_RIGHT_U64;
        assert_eq!(op_unsigned(16, ShiftAmount(2)), Ok(4));
    }
}
