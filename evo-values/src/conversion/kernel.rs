use crate::definitions::failures::ConversionFailure;

// ============================================================================
// 1. Integer -> Float Exactness Helpers
// ============================================================================

#[inline]
pub(crate) fn is_u128_representable_in_f32(m: u128) -> bool {
    if m == 0 {
        return true;
    }
    let k = 128 - m.leading_zeros();
    let tz = m.trailing_zeros();
    (k - tz) <= 24
}

#[inline]
pub(crate) fn is_u128_representable_in_f64(m: u128) -> bool {
    if m == 0 {
        return true;
    }
    let k = 128 - m.leading_zeros();
    let tz = m.trailing_zeros();
    (k - tz) <= 53
}

// ============================================================================
// 2. Float -> Integer Decomposition Helpers
// ============================================================================

/// Decomposes an f32 into (sign, magnitude) if it represents a mathematical integer
/// whose magnitude fits in u128. Returns None if not finite, fractional, or > u128::MAX.
#[inline]
pub(crate) fn decompose_f32_to_integer(val: f32) -> Option<(bool, u128)> {
    let bits = val.to_bits();
    let sign = (bits >> 31) != 0;
    let raw_exp = ((bits >> 23) & 0xff) as i32;
    let raw_mant = (bits & 0x7f_ffff) as u32;

    if raw_exp == 0xff {
        // NaN or Infinity
        return None;
    }
    if raw_exp == 0 {
        if raw_mant == 0 {
            // +0.0 or -0.0
            return Some((false, 0));
        }
        // Subnormal: |val| < 2^-126 < 1, not an integer
        return None;
    }

    let e = raw_exp - 127;
    let m = (1u32 << 23) | raw_mant;

    if e < 0 {
        // 0 < |val| < 1
        None
    } else if e < 23 {
        let shift = (23 - e) as u32;
        let mask = (1u32 << shift) - 1;
        if (m & mask) != 0 {
            // Has fractional bits
            None
        } else {
            Some((sign, (m >> shift) as u128))
        }
    } else {
        let shift = (e - 23) as u32;
        // m has 24 bits. For m << shift to fit in u128: 24 + shift <= 128 => shift <= 104.
        if shift > 104 {
            None
        } else {
            Some((sign, (m as u128) << shift))
        }
    }
}

/// Decomposes an f64 into (sign, magnitude) if it represents a mathematical integer
/// whose magnitude fits in u128. Returns None if not finite, fractional, or > u128::MAX.
#[inline]
pub(crate) fn decompose_f64_to_integer(val: f64) -> Option<(bool, u128)> {
    let bits = val.to_bits();
    let sign = (bits >> 63) != 0;
    let raw_exp = ((bits >> 52) & 0x7ff) as i32;
    let raw_mant = (bits & 0x000f_ffff_ffff_ffff) as u64;

    if raw_exp == 0x7ff {
        // NaN or Infinity
        return None;
    }
    if raw_exp == 0 {
        if raw_mant == 0 {
            // +0.0 or -0.0
            return Some((false, 0));
        }
        // Subnormal: |val| < 2^-1022 < 1, not an integer
        return None;
    }

    let e = raw_exp - 1023;
    let m = (1u64 << 52) | raw_mant;

    if e < 0 {
        // 0 < |val| < 1
        None
    } else if e < 52 {
        let shift = (52 - e) as u32;
        let mask = (1u64 << shift) - 1;
        if (m & mask) != 0 {
            // Has fractional bits
            None
        } else {
            Some((sign, (m >> shift) as u128))
        }
    } else {
        let shift = (e - 52) as u32;
        // m has 53 bits. For m << shift to fit in u128: 53 + shift <= 128 => shift <= 75.
        if shift > 75 {
            None
        } else {
            Some((sign, (m as u128) << shift))
        }
    }
}

// ============================================================================
// 3. Integer Range-Fitting Helpers
// ============================================================================

macro_rules! impl_fit_unsigned {
    ($fn_name:ident, $target:ident) => {
        #[inline]
        pub(crate) fn $fn_name(sign: bool, mag: u128) -> Result<$target, ConversionFailure> {
            if sign && mag != 0 {
                Err(ConversionFailure::NotExactlyRepresentable)
            } else if mag <= $target::MAX as u128 {
                Ok(mag as $target)
            } else {
                Err(ConversionFailure::NotExactlyRepresentable)
            }
        }
    };
}

impl_fit_unsigned!(fit_u8, u8);
impl_fit_unsigned!(fit_u16, u16);
impl_fit_unsigned!(fit_u32, u32);
impl_fit_unsigned!(fit_u64, u64);

#[inline]
pub(crate) fn fit_u128(sign: bool, mag: u128) -> Result<u128, ConversionFailure> {
    if sign && mag != 0 {
        Err(ConversionFailure::NotExactlyRepresentable)
    } else {
        Ok(mag)
    }
}

macro_rules! impl_fit_signed {
    ($fn_name:ident, $target:ident) => {
        #[inline]
        pub(crate) fn $fn_name(sign: bool, mag: u128) -> Result<$target, ConversionFailure> {
            if mag == 0 {
                Ok(0)
            } else if sign {
                let min_abs = $target::MIN.unsigned_abs() as u128;
                if mag == min_abs {
                    Ok($target::MIN)
                } else if mag < min_abs {
                    Ok(-(mag as $target))
                } else {
                    Err(ConversionFailure::NotExactlyRepresentable)
                }
            } else {
                if mag <= $target::MAX as u128 {
                    Ok(mag as $target)
                } else {
                    Err(ConversionFailure::NotExactlyRepresentable)
                }
            }
        }
    };
}

impl_fit_signed!(fit_i8, i8);
impl_fit_signed!(fit_i16, i16);
impl_fit_signed!(fit_i32, i32);
impl_fit_signed!(fit_i64, i64);

#[inline]
pub(crate) fn fit_i128(sign: bool, mag: u128) -> Result<i128, ConversionFailure> {
    if mag == 0 {
        Ok(0)
    } else if sign {
        const I128_MIN_ABS: u128 = 1u128 << 127;
        if mag == I128_MIN_ABS {
            Ok(i128::MIN)
        } else if mag < I128_MIN_ABS {
            Ok(-(mag as i128))
        } else {
            Err(ConversionFailure::NotExactlyRepresentable)
        }
    } else {
        if mag <= i128::MAX as u128 {
            Ok(mag as i128)
        } else {
            Err(ConversionFailure::NotExactlyRepresentable)
        }
    }
}

// ============================================================================
// 4. Float -> Float Exactness Helpers
// ============================================================================

#[inline]
pub(crate) fn f32_to_f64(source: f32) -> Result<f64, ConversionFailure> {
    Ok(source as f64)
}

#[inline]
pub(crate) fn f64_to_f32(source: f64) -> Result<f32, ConversionFailure> {
    if source.is_nan() {
        Ok(f32::NAN)
    } else {
        let candidate = source as f32;
        if (candidate as f64).to_bits() == source.to_bits() {
            Ok(candidate)
        } else {
            Err(ConversionFailure::NotExactlyRepresentable)
        }
    }
}
