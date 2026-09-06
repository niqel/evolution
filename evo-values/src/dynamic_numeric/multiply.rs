use super::integer::{from_big_int, to_big_int};
use crate::definitions::dynamic_numeric::DynamicMultiply;
use crate::definitions::failures::DynamicNumericFailure;
use crate::definitions::value::{DynamicValue, OwnedDynamicValue};

pub fn dynamic_multiply<'left, 'right>(
    left: &DynamicValue<'left>,
    right: &DynamicValue<'right>,
) -> Result<OwnedDynamicValue, DynamicNumericFailure> {
    match (left, right) {
        (DynamicValue::Integer(l), DynamicValue::Integer(r)) => {
            let left_big = to_big_int(l);
            let right_big = to_big_int(r);
            let prod = left_big * right_big;
            Ok(OwnedDynamicValue::Integer(from_big_int(prod)))
        }
        (DynamicValue::Float32(l), DynamicValue::Float32(r)) => {
            Ok(OwnedDynamicValue::Float32(l * r))
        }
        (DynamicValue::Float64(l), DynamicValue::Float64(r)) => {
            Ok(OwnedDynamicValue::Float64(l * r))
        }
        _ => Err(DynamicNumericFailure::DifferentFamily),
    }
}

pub const DYNAMIC_MULTIPLY: DynamicMultiply = dynamic_multiply;

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
    fn test_integer_signs_and_canonical_zero() {
        let pos_7 = make_int(false, &[7]);
        let pos_6 = make_int(false, &[6]);
        let neg_7 = make_int(true, &[7]);
        let neg_6 = make_int(true, &[6]);
        let zero = make_int(false, &[]);

        // positive * positive -> positive
        let res = dynamic_multiply(&pos_7, &pos_6).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[42]);
            }
            _ => panic!("expected Integer"),
        }

        // positive * negative -> negative
        let res = dynamic_multiply(&pos_7, &neg_6).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(owned.negative());
                assert_eq!(owned.magnitude(), &[42]);
            }
            _ => panic!("expected Integer"),
        }

        // negative * positive -> negative
        let res = dynamic_multiply(&neg_7, &pos_6).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(owned.negative());
                assert_eq!(owned.magnitude(), &[42]);
            }
            _ => panic!("expected Integer"),
        }

        // negative * negative -> positive
        let res = dynamic_multiply(&neg_7, &neg_6).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[42]);
            }
            _ => panic!("expected Integer"),
        }

        // zero * N -> canonical zero
        let res = dynamic_multiply(&zero, &pos_7).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[]);
            }
            _ => panic!("expected Integer"),
        }

        // N * zero -> canonical zero
        let res = dynamic_multiply(&neg_7, &zero).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[]);
            }
            _ => panic!("expected Integer"),
        }

        // zero * zero -> canonical zero
        let res = dynamic_multiply(&zero, &zero).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[]);
            }
            _ => panic!("expected Integer"),
        }
    }

    #[test]
    fn test_integer_magnitude_growth() {
        // 0xFF * 0xFF = 0xFE01 (255 * 255 = 65025)
        let val_ff = make_int(false, &[0xFF]);
        let res = dynamic_multiply(&val_ff, &val_ff).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[0xFE, 0x01]);
            }
            _ => panic!("expected Integer"),
        }
    }

    #[test]
    fn test_integer_arbitrary_precision_growth() {
        // Operands > i128 (i128 is 16 bytes; we use 32 bytes = 256 bits)
        let mut op_a = [0x00u8; 32];
        op_a[0] = 0x80; // 2^255
        let mut op_b = [0x00u8; 32];
        op_b[0] = 0x40; // 2^254

        let val_a = make_int(false, &op_a);
        let val_b = make_int(false, &op_b);

        // 2^255 * 2^254 = 2^509
        // Magnitude must have length 64 bytes (> 32 bytes operands, far exceeding i128)
        let res = dynamic_multiply(&val_a, &val_b).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude().len(), 64);
                assert_eq!(owned.magnitude()[0], 0x20); // 2^509: 509 % 8 = 5 -> bit 5 -> 0x20
                for &byte in &owned.magnitude()[1..64] {
                    assert_eq!(byte, 0x00);
                }
            }
            _ => panic!("expected Integer"),
        }
    }

    #[test]
    fn test_float32_cases() {
        // finite * finite
        let res =
            dynamic_multiply(&DynamicValue::Float32(3.5), &DynamicValue::Float32(2.0)).unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => assert_eq!(v, 7.0),
            _ => panic!("expected Float32"),
        }

        // positive * negative
        let res =
            dynamic_multiply(&DynamicValue::Float32(4.0), &DynamicValue::Float32(-2.5)).unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => assert_eq!(v, -10.0),
            _ => panic!("expected Float32"),
        }

        // +0.0 * positive
        let res =
            dynamic_multiply(&DynamicValue::Float32(0.0), &DynamicValue::Float32(5.0)).unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (0.0f32 * 5.0f32).to_bits());
            }
            _ => panic!("expected Float32"),
        }

        // -0.0 * positive
        let res =
            dynamic_multiply(&DynamicValue::Float32(-0.0), &DynamicValue::Float32(5.0)).unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (-0.0f32 * 5.0f32).to_bits());
            }
            _ => panic!("expected Float32"),
        }

        // MAX * MAX -> Infinity
        let res = dynamic_multiply(
            &DynamicValue::Float32(f32::MAX),
            &DynamicValue::Float32(f32::MAX),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (f32::MAX * f32::MAX).to_bits());
            }
            _ => panic!("expected Float32"),
        }

        // Infinity * finite
        let res = dynamic_multiply(
            &DynamicValue::Float32(f32::INFINITY),
            &DynamicValue::Float32(2.0),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (f32::INFINITY * 2.0f32).to_bits());
            }
            _ => panic!("expected Float32"),
        }

        // -Infinity * finite
        let res = dynamic_multiply(
            &DynamicValue::Float32(f32::NEG_INFINITY),
            &DynamicValue::Float32(2.0),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (f32::NEG_INFINITY * 2.0f32).to_bits());
            }
            _ => panic!("expected Float32"),
        }

        // Infinity * 0.0 -> NaN
        let res = dynamic_multiply(
            &DynamicValue::Float32(f32::INFINITY),
            &DynamicValue::Float32(0.0),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert!(v.is_nan());
            }
            _ => panic!("expected Float32"),
        }

        // NaN * finite -> NaN
        let res = dynamic_multiply(
            &DynamicValue::Float32(f32::NAN),
            &DynamicValue::Float32(2.0),
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
        // finite * finite
        let res =
            dynamic_multiply(&DynamicValue::Float64(3.5), &DynamicValue::Float64(2.0)).unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => assert_eq!(v, 7.0),
            _ => panic!("expected Float64"),
        }

        // positive * negative
        let res =
            dynamic_multiply(&DynamicValue::Float64(4.0), &DynamicValue::Float64(-2.5)).unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => assert_eq!(v, -10.0),
            _ => panic!("expected Float64"),
        }

        // +0.0 * positive
        let res =
            dynamic_multiply(&DynamicValue::Float64(0.0), &DynamicValue::Float64(5.0)).unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert_eq!(v.to_bits(), (0.0f64 * 5.0f64).to_bits());
            }
            _ => panic!("expected Float64"),
        }

        // -0.0 * positive
        let res =
            dynamic_multiply(&DynamicValue::Float64(-0.0), &DynamicValue::Float64(5.0)).unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert_eq!(v.to_bits(), (-0.0f64 * 5.0f64).to_bits());
            }
            _ => panic!("expected Float64"),
        }

        // MAX * MAX -> Infinity
        let res = dynamic_multiply(
            &DynamicValue::Float64(f64::MAX),
            &DynamicValue::Float64(f64::MAX),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert_eq!(v.to_bits(), (f64::MAX * f64::MAX).to_bits());
            }
            _ => panic!("expected Float64"),
        }

        // Infinity * finite
        let res = dynamic_multiply(
            &DynamicValue::Float64(f64::INFINITY),
            &DynamicValue::Float64(2.0),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert_eq!(v.to_bits(), (f64::INFINITY * 2.0f64).to_bits());
            }
            _ => panic!("expected Float64"),
        }

        // -Infinity * finite
        let res = dynamic_multiply(
            &DynamicValue::Float64(f64::NEG_INFINITY),
            &DynamicValue::Float64(2.0),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert_eq!(v.to_bits(), (f64::NEG_INFINITY * 2.0f64).to_bits());
            }
            _ => panic!("expected Float64"),
        }

        // Infinity * 0.0 -> NaN
        let res = dynamic_multiply(
            &DynamicValue::Float64(f64::INFINITY),
            &DynamicValue::Float64(0.0),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert!(v.is_nan());
            }
            _ => panic!("expected Float64"),
        }

        // NaN * finite -> NaN
        let res = dynamic_multiply(
            &DynamicValue::Float64(f64::NAN),
            &DynamicValue::Float64(2.0),
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
        let int_val = make_int(false, &[2]);
        let f32_val = DynamicValue::Float32(2.0);
        let f64_val = DynamicValue::Float64(2.0);

        // 1. Integer / Float32
        match dynamic_multiply(&int_val, &f32_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }

        // 2. Integer / Float64
        match dynamic_multiply(&int_val, &f64_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }

        // 3. Float32 / Integer
        match dynamic_multiply(&f32_val, &int_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }

        // 4. Float32 / Float64
        match dynamic_multiply(&f32_val, &f64_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }

        // 5. Float64 / Integer
        match dynamic_multiply(&f64_val, &int_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }

        // 6. Float64 / Float32
        match dynamic_multiply(&f64_val, &f32_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }
    }

    #[test]
    fn test_function_pointer_constant() {
        let op: DynamicMultiply = DYNAMIC_MULTIPLY;

        // Integer multiply
        let a = make_int(false, &[6]);
        let b = make_int(false, &[7]);
        let res_int = op(&a, &b).unwrap();
        match res_int {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[42]);
            }
            _ => panic!("expected Integer"),
        }

        // Float multiply
        let f1 = DynamicValue::Float32(2.5);
        let f2 = DynamicValue::Float32(4.0);
        let res_float = op(&f1, &f2).unwrap();
        match res_float {
            OwnedDynamicValue::Float32(v) => assert_eq!(v, 10.0),
            _ => panic!("expected Float32"),
        }

        // DifferentFamily
        match op(&a, &f1) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }
    }
}
