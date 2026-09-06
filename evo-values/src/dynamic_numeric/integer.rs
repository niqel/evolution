use num_bigint::{BigInt, Sign};

use crate::definitions::value::{DynamicIntegerValue, OwnedDynamicInteger};

#[allow(dead_code)]
pub(crate) fn to_big_int(value: &DynamicIntegerValue<'_>) -> BigInt {
    let magnitude = value.magnitude();
    if magnitude.is_empty() {
        BigInt::from_bytes_be(Sign::NoSign, &[])
    } else if value.negative() {
        BigInt::from_bytes_be(Sign::Minus, magnitude)
    } else {
        BigInt::from_bytes_be(Sign::Plus, magnitude)
    }
}

#[allow(dead_code)]
pub(crate) fn from_big_int(value: BigInt) -> OwnedDynamicInteger {
    let (sign, magnitude) = value.to_bytes_be();
    let negative = sign == Sign::Minus;
    OwnedDynamicInteger::from_parts(negative, magnitude.into_boxed_slice())
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::borrow::Cow;

    #[test]
    fn test_zero_round_trip() {
        let val_zero = DynamicIntegerValue::from_parts(false, Cow::Borrowed(&[]));
        let big = to_big_int(&val_zero);
        assert_eq!(big.sign(), Sign::NoSign);
        assert_eq!(big, BigInt::from_bytes_be(Sign::NoSign, &[]));

        let owned = from_big_int(big);
        assert!(!owned.negative());
        assert_eq!(owned.magnitude(), &[]);

        let val_zero_zeros =
            DynamicIntegerValue::from_parts(true, Cow::Borrowed(&[0x00, 0x00, 0x00]));
        let big_zeros = to_big_int(&val_zero_zeros);
        assert_eq!(big_zeros.sign(), Sign::NoSign);
        let owned_zeros = from_big_int(big_zeros);
        assert!(!owned_zeros.negative());
        assert_eq!(owned_zeros.magnitude(), &[]);
    }

    #[test]
    fn test_positive_multi_byte_round_trip() {
        let magnitude = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
        let val = DynamicIntegerValue::from_parts(false, Cow::Borrowed(&magnitude));
        let big = to_big_int(&val);
        assert_eq!(big.sign(), Sign::Plus);

        let owned = from_big_int(big);
        assert!(!owned.negative());
        assert_eq!(owned.magnitude(), &magnitude);
    }

    #[test]
    fn test_negative_multi_byte_round_trip() {
        let magnitude = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF];
        let val = DynamicIntegerValue::from_parts(true, Cow::Borrowed(&magnitude));
        let big = to_big_int(&val);
        assert_eq!(big.sign(), Sign::Minus);

        let owned = from_big_int(big);
        assert!(owned.negative());
        assert_eq!(owned.magnitude(), &magnitude);
    }

    #[test]
    fn test_very_large_magnitude_round_trip() {
        // 64 bytes = 512 bits, far exceeding i128 (16 bytes = 128 bits)
        let mut large_magnitude = [0u8; 64];
        for (i, byte) in large_magnitude.iter_mut().enumerate() {
            *byte = ((i * 7 + 13) % 255 + 1) as u8;
        }

        // Positive very large integer
        let val_pos = DynamicIntegerValue::from_parts(false, Cow::Borrowed(&large_magnitude));
        let big_pos = to_big_int(&val_pos);
        assert_eq!(big_pos.sign(), Sign::Plus);
        let owned_pos = from_big_int(big_pos);
        assert!(!owned_pos.negative());
        assert_eq!(owned_pos.magnitude(), &large_magnitude[..]);

        // Negative very large integer
        let val_neg = DynamicIntegerValue::from_parts(true, Cow::Borrowed(&large_magnitude));
        let big_neg = to_big_int(&val_neg);
        assert_eq!(big_neg.sign(), Sign::Minus);
        let owned_neg = from_big_int(big_neg);
        assert!(owned_neg.negative());
        assert_eq!(owned_neg.magnitude(), &large_magnitude[..]);
    }

    #[test]
    fn test_canonicalization() {
        // Direct BigInt zero
        let big_zero = BigInt::from_bytes_be(Sign::NoSign, &[]);
        let owned_zero = from_big_int(big_zero);
        assert!(!owned_zero.negative());
        assert_eq!(owned_zero.magnitude(), &[]);

        // Non-zero BigInt should never produce leading zeros or negative zero
        let big_one = BigInt::from_bytes_be(Sign::Plus, &[0x01]);
        let owned_one = from_big_int(big_one);
        assert!(!owned_one.negative());
        assert_eq!(owned_one.magnitude(), &[0x01]);

        let big_neg_one = BigInt::from_bytes_be(Sign::Minus, &[0x01]);
        let owned_neg_one = from_big_int(big_neg_one);
        assert!(owned_neg_one.negative());
        assert_eq!(owned_neg_one.magnitude(), &[0x01]);
    }
}
