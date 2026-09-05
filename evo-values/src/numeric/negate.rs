use crate::definitions::failures::NumericFailure;
use crate::definitions::numeric::negate::Negate;

macro_rules! impl_negate {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(val: $t) -> Result<$t, NumericFailure> {
            val.checked_neg().ok_or(NumericFailure::Overflow)
        }

        pub const $const_name: Negate<$t> = $fn_name;
    };
}

impl_negate!(negate_i8, NEGATE_I8, i8);
impl_negate!(negate_i16, NEGATE_I16, i16);
impl_negate!(negate_i32, NEGATE_I32, i32);
impl_negate!(negate_i64, NEGATE_I64, i64);
impl_negate!(negate_i128, NEGATE_I128, i128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negate_signed_cases() {
        assert_eq!(negate_i8(5), Ok(-5));
        assert_eq!(negate_i8(-5), Ok(5));
        assert_eq!(negate_i8(0), Ok(0));
        assert_eq!(negate_i8(i8::MIN), Err(NumericFailure::Overflow));

        assert_eq!(negate_i16(i16::MIN), Err(NumericFailure::Overflow));
        assert_eq!(negate_i32(i32::MIN), Err(NumericFailure::Overflow));
        assert_eq!(negate_i64(i64::MIN), Err(NumericFailure::Overflow));
        assert_eq!(negate_i128(i128::MIN), Err(NumericFailure::Overflow));
    }

    #[test]
    fn negate_constants() {
        let op: Negate<i32> = NEGATE_I32;
        assert_eq!(op(42), Ok(-42));
    }
}
