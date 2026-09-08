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
