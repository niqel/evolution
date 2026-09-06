use super::integer::{from_big_int, to_big_int};
use crate::definitions::dynamic_numeric::DynamicAdd;
use crate::definitions::failures::DynamicNumericFailure;
use crate::definitions::value::{DynamicValue, OwnedDynamicValue};

pub fn dynamic_add<'left, 'right>(
    left: &DynamicValue<'left>,
    right: &DynamicValue<'right>,
) -> Result<OwnedDynamicValue, DynamicNumericFailure> {
    match (left, right) {
        (DynamicValue::Integer(l), DynamicValue::Integer(r)) => {
            let left_big = to_big_int(l);
            let right_big = to_big_int(r);
            let sum = left_big + right_big;
            Ok(OwnedDynamicValue::Integer(from_big_int(sum)))
        }
        (DynamicValue::Float32(l), DynamicValue::Float32(r)) => {
            Ok(OwnedDynamicValue::Float32(l + r))
        }
        (DynamicValue::Float64(l), DynamicValue::Float64(r)) => {
            Ok(OwnedDynamicValue::Float64(l + r))
        }
        _ => Err(DynamicNumericFailure::DifferentFamily),
    }
}

pub const DYNAMIC_ADD: DynamicAdd = dynamic_add;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::value::DynamicIntegerValue;
    use alloc::borrow::Cow;

    fn make_int(negative: bool, magnitude: &[u8]) -> DynamicValue<'_> {
        DynamicValue::Integer(DynamicIntegerValue::from_parts(
            negative,
            Cow::Borrowed(magnitude),
        ))
    }

    #[test]
    fn test_integer_zero() {
        let zero = make_int(false, &[]);
        let pos = make_int(false, &[0x2A]);

        // 0 + 0 = 0
        let res = dynamic_add(&zero, &zero).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[]);
            }
            _ => panic!("expected Integer"),
        }

        // 0 + N = N
        let res = dynamic_add(&zero, &pos).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[0x2A]);
            }
            _ => panic!("expected Integer"),
        }

        // N + 0 = N
        let res = dynamic_add(&pos, &zero).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[0x2A]);
            }
            _ => panic!("expected Integer"),
        }
    }

    #[test]
    fn test_integer_same_sign_positive_and_carry_growth() {
        let a = make_int(false, &[0x12, 0x34]);
        let b = make_int(false, &[0x01, 0x00]);
        let res = dynamic_add(&a, &b).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[0x13, 0x34]);
            }
            _ => panic!("expected Integer"),
        }

        // Carry and magnitude growth: 0xFF + 0x01 = 0x0100
        let ff = make_int(false, &[0xFF]);
        let one = make_int(false, &[0x01]);
        let res = dynamic_add(&ff, &one).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[0x01, 0x00]);
            }
            _ => panic!("expected Integer"),
        }

        // Multi-byte carry growth: [0xFF, 0xFF] + 1 = [0x01, 0x00, 0x00]
        let ffff = make_int(false, &[0xFF, 0xFF]);
        let res2 = dynamic_add(&ffff, &one).unwrap();
        match res2 {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[0x01, 0x00, 0x00]);
            }
            _ => panic!("expected Integer"),
        }
    }

    #[test]
    fn test_integer_same_sign_negative() {
        let neg_a = make_int(true, &[0x12, 0x34]);
        let neg_b = make_int(true, &[0x01, 0x00]);
        let res = dynamic_add(&neg_a, &neg_b).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(owned.negative());
                assert_eq!(owned.magnitude(), &[0x13, 0x34]);
            }
            _ => panic!("expected Integer"),
        }

        // Negative with carry: (-0xFF) + (-0x01) = -0x0100
        let neg_ff = make_int(true, &[0xFF]);
        let neg_one = make_int(true, &[0x01]);
        let res_carry = dynamic_add(&neg_ff, &neg_one).unwrap();
        match res_carry {
            OwnedDynamicValue::Integer(owned) => {
                assert!(owned.negative());
                assert_eq!(owned.magnitude(), &[0x01, 0x00]);
            }
            _ => panic!("expected Integer"),
        }
    }

    #[test]
    fn test_integer_opposite_signs() {
        // larger positive + smaller negative -> positive: 10 + (-3) = 7
        let pos_10 = make_int(false, &[10]);
        let neg_3 = make_int(true, &[3]);
        let res1 = dynamic_add(&pos_10, &neg_3).unwrap();
        match res1 {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[7]);
            }
            _ => panic!("expected Integer"),
        }

        // smaller positive + larger negative -> negative: 3 + (-10) = -7
        let pos_3 = make_int(false, &[3]);
        let neg_10 = make_int(true, &[10]);
        let res2 = dynamic_add(&pos_3, &neg_10).unwrap();
        match res2 {
            OwnedDynamicValue::Integer(owned) => {
                assert!(owned.negative());
                assert_eq!(owned.magnitude(), &[7]);
            }
            _ => panic!("expected Integer"),
        }

        // N + (-N) -> canonical zero
        let pos_val = make_int(false, &[0xDE, 0xAD, 0xBE, 0xEF]);
        let neg_val = make_int(true, &[0xDE, 0xAD, 0xBE, 0xEF]);
        let res_zero = dynamic_add(&pos_val, &neg_val).unwrap();
        match res_zero {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[]);
            }
            _ => panic!("expected Integer"),
        }
    }

    #[test]
    fn test_integer_arbitrary_precision() {
        // 64 bytes = 512 bits, far exceeding i128
        let mut large_a = [0xFFu8; 64];
        large_a[0] = 0x7F; // highest byte
        let mut large_b = [0u8; 64];
        large_b[63] = 0x02; // lowest byte = 2

        let val_a = make_int(false, &large_a);
        let val_b = make_int(false, &large_b);

        let res = dynamic_add(&val_a, &val_b).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                // [0x7F, 0xFF, ..., 0xFF] + 2 = [0x80, 0x00, ..., 0x01]
                assert_eq!(owned.magnitude().len(), 64);
                assert_eq!(owned.magnitude()[0], 0x80);
                for &byte in &owned.magnitude()[1..63] {
                    assert_eq!(byte, 0x00);
                }
                assert_eq!(owned.magnitude()[63], 0x01);
            }
            _ => panic!("expected Integer"),
        }
    }

    #[test]
    fn test_float32_cases() {
        // normal finite addition
        let res = dynamic_add(&DynamicValue::Float32(1.5), &DynamicValue::Float32(2.25)).unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => assert_eq!(v, 3.75),
            _ => panic!("expected Float32"),
        }

        // positive + negative
        let res = dynamic_add(&DynamicValue::Float32(5.0), &DynamicValue::Float32(-2.0)).unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => assert_eq!(v, 3.0),
            _ => panic!("expected Float32"),
        }

        // +0.0 / -0.0 behavior
        let zero_cases = [
            (0.0f32, 0.0f32),
            (0.0f32, -0.0f32),
            (-0.0f32, 0.0f32),
            (-0.0f32, -0.0f32),
        ];
        for (l, r) in zero_cases {
            let res = dynamic_add(&DynamicValue::Float32(l), &DynamicValue::Float32(r)).unwrap();
            match res {
                OwnedDynamicValue::Float32(v) => assert_eq!(v.to_bits(), (l + r).to_bits()),
                _ => panic!("expected Float32"),
            }
        }

        // MAX + MAX -> Infinity
        let res = dynamic_add(
            &DynamicValue::Float32(f32::MAX),
            &DynamicValue::Float32(f32::MAX),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (f32::MAX + f32::MAX).to_bits())
            }
            _ => panic!("expected Float32"),
        }

        // Infinity + finite
        let res = dynamic_add(
            &DynamicValue::Float32(f32::INFINITY),
            &DynamicValue::Float32(1.0),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (f32::INFINITY + 1.0f32).to_bits())
            }
            _ => panic!("expected Float32"),
        }

        // -Infinity + finite
        let res = dynamic_add(
            &DynamicValue::Float32(f32::NEG_INFINITY),
            &DynamicValue::Float32(1.0),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (f32::NEG_INFINITY + 1.0f32).to_bits())
            }
            _ => panic!("expected Float32"),
        }

        // Infinity + -Infinity -> NaN
        let res = dynamic_add(
            &DynamicValue::Float32(f32::INFINITY),
            &DynamicValue::Float32(f32::NEG_INFINITY),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert!(v.is_nan());
            }
            _ => panic!("expected Float32"),
        }

        // NaN + finite -> NaN
        let res = dynamic_add(
            &DynamicValue::Float32(f32::NAN),
            &DynamicValue::Float32(1.0),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert!(v.is_nan());
            }
            _ => panic!("expected Float32"),
        }
    }

    #[test]
    fn test_float64_cases() {
        // normal finite addition
        let res = dynamic_add(&DynamicValue::Float64(1.5), &DynamicValue::Float64(2.25)).unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => assert_eq!(v, 3.75),
            _ => panic!("expected Float64"),
        }

        // positive + negative
        let res = dynamic_add(&DynamicValue::Float64(5.0), &DynamicValue::Float64(-2.0)).unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => assert_eq!(v, 3.0),
            _ => panic!("expected Float64"),
        }

        // +0.0 / -0.0 behavior
        let zero_cases = [
            (0.0f64, 0.0f64),
            (0.0f64, -0.0f64),
            (-0.0f64, 0.0f64),
            (-0.0f64, -0.0f64),
        ];
        for (l, r) in zero_cases {
            let res = dynamic_add(&DynamicValue::Float64(l), &DynamicValue::Float64(r)).unwrap();
            match res {
                OwnedDynamicValue::Float64(v) => assert_eq!(v.to_bits(), (l + r).to_bits()),
                _ => panic!("expected Float64"),
            }
        }

        // MAX + MAX -> Infinity
        let res = dynamic_add(
            &DynamicValue::Float64(f64::MAX),
            &DynamicValue::Float64(f64::MAX),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert_eq!(v.to_bits(), (f64::MAX + f64::MAX).to_bits())
            }
            _ => panic!("expected Float64"),
        }

        // Infinity + finite
        let res = dynamic_add(
            &DynamicValue::Float64(f64::INFINITY),
            &DynamicValue::Float64(1.0),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert_eq!(v.to_bits(), (f64::INFINITY + 1.0f64).to_bits())
            }
            _ => panic!("expected Float64"),
        }

        // -Infinity + finite
        let res = dynamic_add(
            &DynamicValue::Float64(f64::NEG_INFINITY),
            &DynamicValue::Float64(1.0),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert_eq!(v.to_bits(), (f64::NEG_INFINITY + 1.0f64).to_bits())
            }
            _ => panic!("expected Float64"),
        }

        // Infinity + -Infinity -> NaN
        let res = dynamic_add(
            &DynamicValue::Float64(f64::INFINITY),
            &DynamicValue::Float64(f64::NEG_INFINITY),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert!(v.is_nan());
            }
            _ => panic!("expected Float64"),
        }

        // NaN + finite -> NaN
        let res = dynamic_add(
            &DynamicValue::Float64(f64::NAN),
            &DynamicValue::Float64(1.0),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert!(v.is_nan());
            }
            _ => panic!("expected Float64"),
        }
    }

    #[test]
    fn test_different_family_all_pairs() {
        let int_val = make_int(false, &[1]);
        let f32_val = DynamicValue::Float32(1.0);
        let f64_val = DynamicValue::Float64(1.0);

        // 1. Integer / Float32
        match dynamic_add(&int_val, &f32_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }

        // 2. Integer / Float64
        match dynamic_add(&int_val, &f64_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }

        // 3. Float32 / Integer
        match dynamic_add(&f32_val, &int_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }

        // 4. Float32 / Float64
        match dynamic_add(&f32_val, &f64_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }

        // 5. Float64 / Integer
        match dynamic_add(&f64_val, &int_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }

        // 6. Float64 / Float32
        match dynamic_add(&f64_val, &f32_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }
    }

    #[test]
    fn test_function_pointer_constant() {
        let op: DynamicAdd = DYNAMIC_ADD;

        // Integer add
        let a = make_int(false, &[10]);
        let b = make_int(false, &[20]);
        let res_int = op(&a, &b).unwrap();
        match res_int {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[30]);
            }
            _ => panic!("expected Integer"),
        }

        // Float add
        let f1 = DynamicValue::Float32(1.25);
        let f2 = DynamicValue::Float32(2.5);
        let res_float = op(&f1, &f2).unwrap();
        match res_float {
            OwnedDynamicValue::Float32(v) => assert_eq!(v, 3.75),
            _ => panic!("expected Float32"),
        }

        // DifferentFamily
        match op(&a, &f1) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }
    }
}
