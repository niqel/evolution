use crate::definitions::failures::NumericFailure;
use crate::definitions::numeric::clamp::{FloatClamp, IntegerClamp};

macro_rules! impl_clamp {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(value: $t, minimum: $t, maximum: $t) -> Result<$t, NumericFailure> {
            if minimum > maximum {
                return Err(NumericFailure::InvalidBounds);
            }
            if value < minimum {
                Ok(minimum)
            } else if value > maximum {
                Ok(maximum)
            } else {
                Ok(value)
            }
        }

        pub const $const_name: IntegerClamp<$t> = $fn_name;
    };
}

impl_clamp!(clamp_i8, CLAMP_I8, i8);
impl_clamp!(clamp_i16, CLAMP_I16, i16);
impl_clamp!(clamp_i32, CLAMP_I32, i32);
impl_clamp!(clamp_i64, CLAMP_I64, i64);
impl_clamp!(clamp_i128, CLAMP_I128, i128);

impl_clamp!(clamp_u8, CLAMP_U8, u8);
impl_clamp!(clamp_u16, CLAMP_U16, u16);
impl_clamp!(clamp_u32, CLAMP_U32, u32);
impl_clamp!(clamp_u64, CLAMP_U64, u64);
impl_clamp!(clamp_u128, CLAMP_U128, u128);

macro_rules! impl_float_clamp {
    ($fn_name:ident, $const_name:ident, $t:ty) => {
        pub fn $fn_name(value: $t, minimum: $t, maximum: $t) -> Result<$t, NumericFailure> {
            if minimum > maximum || minimum.is_nan() || maximum.is_nan() {
                return Err(NumericFailure::InvalidBounds);
            }
            if value < minimum {
                Ok(minimum)
            } else if value > maximum {
                Ok(maximum)
            } else {
                Ok(value)
            }
        }

        pub const $const_name: FloatClamp<$t> = $fn_name;
    };
}

impl_float_clamp!(clamp_f32, CLAMP_F32, f32);
impl_float_clamp!(clamp_f64, CLAMP_F64, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_signed() {
        assert_eq!(clamp_i32(5, 0, 10), Ok(5));
        assert_eq!(clamp_i32(-5, 0, 10), Ok(0));
        assert_eq!(clamp_i32(20, 0, 10), Ok(10));
        assert_eq!(clamp_i32(0, 0, 10), Ok(0));
        assert_eq!(clamp_i32(10, 0, 10), Ok(10));
        assert_eq!(clamp_i32(100, 7, 7), Ok(7));
        assert_eq!(clamp_i32(5, 10, 0), Err(NumericFailure::InvalidBounds));
    }

    #[test]
    fn clamp_unsigned() {
        assert_eq!(clamp_u32(5, 0, 10), Ok(5));
        assert_eq!(clamp_u32(0, 0, 10), Ok(0));
        assert_eq!(clamp_u32(20, 0, 10), Ok(10));
        assert_eq!(clamp_u32(100, 7, 7), Ok(7));
        assert_eq!(clamp_u32(5, 10, 0), Err(NumericFailure::InvalidBounds));
    }

    #[test]
    fn clamp_float() {
        assert_eq!(clamp_f32(5.0, 0.0, 10.0), Ok(5.0));
        assert_eq!(clamp_f32(-5.0, 0.0, 10.0), Ok(0.0));
        assert_eq!(clamp_f32(20.0, 0.0, 10.0), Ok(10.0));
        assert_eq!(
            clamp_f32(5.0, 10.0, 0.0),
            Err(NumericFailure::InvalidBounds)
        );
        assert_eq!(
            clamp_f32(5.0, f32::NAN, 10.0),
            Err(NumericFailure::InvalidBounds)
        );
        assert_eq!(
            clamp_f32(5.0, 0.0, f32::NAN),
            Err(NumericFailure::InvalidBounds)
        );
        let nan_res = clamp_f32(f32::NAN, 0.0, 10.0).unwrap();
        assert!(nan_res.is_nan());

        assert_eq!(clamp_f64(5.0, 0.0, 10.0), Ok(5.0));
        assert_eq!(clamp_f64(-5.0, 0.0, 10.0), Ok(0.0));
        assert_eq!(clamp_f64(20.0, 0.0, 10.0), Ok(10.0));
        assert_eq!(
            clamp_f64(5.0, 10.0, 0.0),
            Err(NumericFailure::InvalidBounds)
        );
        assert_eq!(
            clamp_f64(5.0, f64::NAN, 10.0),
            Err(NumericFailure::InvalidBounds)
        );
        assert_eq!(
            clamp_f64(5.0, 0.0, f64::NAN),
            Err(NumericFailure::InvalidBounds)
        );
        let nan_res64 = clamp_f64(f64::NAN, 0.0, 10.0).unwrap();
        assert!(nan_res64.is_nan());
    }

    #[test]
    fn clamp_constants() {
        let op: IntegerClamp<i128> = CLAMP_I128;
        assert_eq!(op(5, 0, 10), Ok(5));

        let op_float: FloatClamp<f64> = CLAMP_F64;
        assert_eq!(op_float(5.0, 0.0, 10.0), Ok(5.0));
    }
}
