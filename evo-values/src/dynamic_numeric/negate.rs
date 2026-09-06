use alloc::boxed::Box;

use crate::definitions::dynamic_numeric::DynamicNegate;
use crate::definitions::value::{DynamicValue, OwnedDynamicInteger, OwnedDynamicValue};

pub fn dynamic_negate<'value>(value: &DynamicValue<'value>) -> OwnedDynamicValue {
    match value {
        DynamicValue::Integer(val) => {
            let magnitude = val.magnitude();
            if magnitude.is_empty() {
                OwnedDynamicValue::Integer(OwnedDynamicInteger::from_parts(false, Box::new([])))
            } else {
                let negative = !val.negative();
                OwnedDynamicValue::Integer(OwnedDynamicInteger::from_parts(
                    negative,
                    Box::from(magnitude),
                ))
            }
        }
        DynamicValue::Float32(val) => OwnedDynamicValue::Float32(-val),
        DynamicValue::Float64(val) => OwnedDynamicValue::Float64(-val),
    }
}

pub const DYNAMIC_NEGATE: DynamicNegate = dynamic_negate;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::value::DynamicIntegerValue;
    use alloc::borrow::Cow;

    #[test]
    fn test_integer_positive() {
        let val = DynamicValue::Integer(DynamicIntegerValue::from_parts(
            false,
            Cow::Borrowed(&[0x01, 0x23, 0x45]),
        ));
        let res = dynamic_negate(&val);
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(owned.negative());
                assert_eq!(owned.magnitude(), &[0x01, 0x23, 0x45]);
            }
            _ => panic!("expected OwnedDynamicValue::Integer"),
        }
    }

    #[test]
    fn test_integer_negative() {
        let val = DynamicValue::Integer(DynamicIntegerValue::from_parts(
            true,
            Cow::Borrowed(&[0x01, 0x23, 0x45]),
        ));
        let res = dynamic_negate(&val);
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[0x01, 0x23, 0x45]);
            }
            _ => panic!("expected OwnedDynamicValue::Integer"),
        }
    }

    #[test]
    fn test_integer_zero() {
        let val_zero =
            DynamicValue::Integer(DynamicIntegerValue::from_parts(false, Cow::Borrowed(&[])));
        let res = dynamic_negate(&val_zero);
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[]);
            }
            _ => panic!("expected OwnedDynamicValue::Integer"),
        }

        // Zero constructed with negative = true and zeros
        let val_zero_neg = DynamicValue::Integer(DynamicIntegerValue::from_parts(
            true,
            Cow::Borrowed(&[0x00, 0x00]),
        ));
        let res_neg = dynamic_negate(&val_zero_neg);
        match res_neg {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[]);
            }
            _ => panic!("expected OwnedDynamicValue::Integer"),
        }
    }

    #[test]
    fn test_very_large_integer() {
        // 64 bytes = 512 bits, far larger than i128 (16 bytes = 128 bits)
        let mut large_magnitude = [0u8; 64];
        for (i, byte) in large_magnitude.iter_mut().enumerate() {
            *byte = ((i * 11 + 17) % 255 + 1) as u8;
        }

        // Positive -> Negative
        let val_pos = DynamicValue::Integer(DynamicIntegerValue::from_parts(
            false,
            Cow::Borrowed(&large_magnitude),
        ));
        let res_pos = dynamic_negate(&val_pos);
        match res_pos {
            OwnedDynamicValue::Integer(owned) => {
                assert!(owned.negative());
                assert_eq!(owned.magnitude(), &large_magnitude[..]);
            }
            _ => panic!("expected OwnedDynamicValue::Integer"),
        }

        // Negative -> Positive
        let val_neg = DynamicValue::Integer(DynamicIntegerValue::from_parts(
            true,
            Cow::Borrowed(&large_magnitude),
        ));
        let res_neg = dynamic_negate(&val_neg);
        match res_neg {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &large_magnitude[..]);
            }
            _ => panic!("expected OwnedDynamicValue::Integer"),
        }
    }

    #[test]
    fn test_float32_cases() {
        // positive
        let res = dynamic_negate(&DynamicValue::Float32(123.456));
        match res {
            OwnedDynamicValue::Float32(val) => assert_eq!(val, -123.456),
            _ => panic!("expected Float32"),
        }

        // negative
        let res = dynamic_negate(&DynamicValue::Float32(-123.456));
        match res {
            OwnedDynamicValue::Float32(val) => assert_eq!(val, 123.456),
            _ => panic!("expected Float32"),
        }

        // +0.0 -> -0.0
        let res = dynamic_negate(&DynamicValue::Float32(0.0));
        match res {
            OwnedDynamicValue::Float32(val) => assert_eq!(val.to_bits(), (-0.0f32).to_bits()),
            _ => panic!("expected Float32"),
        }

        // -0.0 -> +0.0
        let res = dynamic_negate(&DynamicValue::Float32(-0.0));
        match res {
            OwnedDynamicValue::Float32(val) => assert_eq!(val.to_bits(), (0.0f32).to_bits()),
            _ => panic!("expected Float32"),
        }

        // +Infinity -> -Infinity
        let res = dynamic_negate(&DynamicValue::Float32(f32::INFINITY));
        match res {
            OwnedDynamicValue::Float32(val) => assert_eq!(val, f32::NEG_INFINITY),
            _ => panic!("expected Float32"),
        }

        // -Infinity -> +Infinity
        let res = dynamic_negate(&DynamicValue::Float32(f32::NEG_INFINITY));
        match res {
            OwnedDynamicValue::Float32(val) => assert_eq!(val, f32::INFINITY),
            _ => panic!("expected Float32"),
        }

        // NaN
        let nan_in = f32::NAN;
        let res = dynamic_negate(&DynamicValue::Float32(nan_in));
        match res {
            OwnedDynamicValue::Float32(val) => {
                assert!(val.is_nan());
                assert_eq!(val.to_bits(), (-nan_in).to_bits());
            }
            _ => panic!("expected Float32"),
        }
    }

    #[test]
    fn test_float64_cases() {
        // positive
        let res = dynamic_negate(&DynamicValue::Float64(123.4567890123));
        match res {
            OwnedDynamicValue::Float64(val) => assert_eq!(val, -123.4567890123),
            _ => panic!("expected Float64"),
        }

        // negative
        let res = dynamic_negate(&DynamicValue::Float64(-123.4567890123));
        match res {
            OwnedDynamicValue::Float64(val) => assert_eq!(val, 123.4567890123),
            _ => panic!("expected Float64"),
        }

        // +0.0 -> -0.0
        let res = dynamic_negate(&DynamicValue::Float64(0.0));
        match res {
            OwnedDynamicValue::Float64(val) => assert_eq!(val.to_bits(), (-0.0f64).to_bits()),
            _ => panic!("expected Float64"),
        }

        // -0.0 -> +0.0
        let res = dynamic_negate(&DynamicValue::Float64(-0.0));
        match res {
            OwnedDynamicValue::Float64(val) => assert_eq!(val.to_bits(), (0.0f64).to_bits()),
            _ => panic!("expected Float64"),
        }

        // +Infinity -> -Infinity
        let res = dynamic_negate(&DynamicValue::Float64(f64::INFINITY));
        match res {
            OwnedDynamicValue::Float64(val) => assert_eq!(val, f64::NEG_INFINITY),
            _ => panic!("expected Float64"),
        }

        // -Infinity -> +Infinity
        let res = dynamic_negate(&DynamicValue::Float64(f64::NEG_INFINITY));
        match res {
            OwnedDynamicValue::Float64(val) => assert_eq!(val, f64::INFINITY),
            _ => panic!("expected Float64"),
        }

        // NaN
        let nan_in = f64::NAN;
        let res = dynamic_negate(&DynamicValue::Float64(nan_in));
        match res {
            OwnedDynamicValue::Float64(val) => {
                assert!(val.is_nan());
                assert_eq!(val.to_bits(), (-nan_in).to_bits());
            }
            _ => panic!("expected Float64"),
        }
    }

    #[test]
    fn test_function_pointer_constant() {
        let op: DynamicNegate = DYNAMIC_NEGATE;

        // Execute via op(...) for Integer
        let val_int = DynamicValue::Integer(DynamicIntegerValue::from_parts(
            false,
            Cow::Borrowed(&[0x42]),
        ));
        let res_int = op(&val_int);
        match res_int {
            OwnedDynamicValue::Integer(owned) => {
                assert!(owned.negative());
                assert_eq!(owned.magnitude(), &[0x42]);
            }
            _ => panic!("expected Integer"),
        }

        // Execute via op(...) for Float32
        let val_f32 = DynamicValue::Float32(3.14);
        let res_f32 = op(&val_f32);
        match res_f32 {
            OwnedDynamicValue::Float32(v) => assert_eq!(v, -3.14),
            _ => panic!("expected Float32"),
        }

        // Execute via op(...) for Float64
        let val_f64 = DynamicValue::Float64(-2.718);
        let res_f64 = op(&val_f64);
        match res_f64 {
            OwnedDynamicValue::Float64(v) => assert_eq!(v, 2.718),
            _ => panic!("expected Float64"),
        }
    }
}
