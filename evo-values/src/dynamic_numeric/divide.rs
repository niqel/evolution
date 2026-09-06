use super::integer::{from_big_int, to_big_int};
use crate::definitions::dynamic_numeric::DynamicDivide;
use crate::definitions::failures::DynamicNumericFailure;
use crate::definitions::value::{DynamicValue, OwnedDynamicValue};

pub fn dynamic_divide<'left, 'right>(
    left: &DynamicValue<'left>,
    right: &DynamicValue<'right>,
) -> Result<OwnedDynamicValue, DynamicNumericFailure> {
    match (left, right) {
        (DynamicValue::Integer(l), DynamicValue::Integer(r)) => {
            if r.magnitude().is_empty() {
                return Err(DynamicNumericFailure::DivisionByZero);
            }
            let left_big = to_big_int(l);
            let right_big = to_big_int(r);
            let quotient = left_big / right_big;
            Ok(OwnedDynamicValue::Integer(from_big_int(quotient)))
        }
        (DynamicValue::Float32(l), DynamicValue::Float32(r)) => {
            Ok(OwnedDynamicValue::Float32(l / r))
        }
        (DynamicValue::Float64(l), DynamicValue::Float64(r)) => {
            Ok(OwnedDynamicValue::Float64(l / r))
        }
        _ => Err(DynamicNumericFailure::DifferentFamily),
    }
}

pub const DYNAMIC_DIVIDE: DynamicDivide = dynamic_divide;

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
    fn test_integer_truncation_toward_zero() {
        let pos_7 = make_int(false, &[7]);
        let pos_3 = make_int(false, &[3]);
        let neg_7 = make_int(true, &[7]);
        let neg_3 = make_int(true, &[3]);

        //  7 /  3 =  2
        let res = dynamic_divide(&pos_7, &pos_3).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[2]);
            }
            _ => panic!("expected Integer"),
        }

        // -7 /  3 = -2
        let res = dynamic_divide(&neg_7, &pos_3).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(owned.negative());
                assert_eq!(owned.magnitude(), &[2]);
            }
            _ => panic!("expected Integer"),
        }

        //  7 / -3 = -2
        let res = dynamic_divide(&pos_7, &neg_3).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(owned.negative());
                assert_eq!(owned.magnitude(), &[2]);
            }
            _ => panic!("expected Integer"),
        }

        // -7 / -3 =  2
        let res = dynamic_divide(&neg_7, &neg_3).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[2]);
            }
            _ => panic!("expected Integer"),
        }
    }

    #[test]
    fn test_integer_special_cases() {
        let zero = make_int(false, &[]);
        let pos_10 = make_int(false, &[10]);
        let one = make_int(false, &[1]);
        let neg_one = make_int(true, &[1]);

        // 0 / N = 0 (canonical zero)
        let res = dynamic_divide(&zero, &pos_10).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[]);
            }
            _ => panic!("expected Integer"),
        }

        // N / 1 = N
        let res = dynamic_divide(&pos_10, &one).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[10]);
            }
            _ => panic!("expected Integer"),
        }

        // N / -1 = -N
        let res = dynamic_divide(&pos_10, &neg_one).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(owned.negative());
                assert_eq!(owned.magnitude(), &[10]);
            }
            _ => panic!("expected Integer"),
        }
    }

    #[test]
    fn test_integer_division_by_zero() {
        let pos_7 = make_int(false, &[7]);
        let neg_7 = make_int(true, &[7]);
        let zero = make_int(false, &[]);

        // positive / 0
        match dynamic_divide(&pos_7, &zero) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DivisionByZero),
            _ => panic!("expected DivisionByZero"),
        }

        // negative / 0
        match dynamic_divide(&neg_7, &zero) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DivisionByZero),
            _ => panic!("expected DivisionByZero"),
        }

        // 0 / 0
        match dynamic_divide(&zero, &zero) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DivisionByZero),
            _ => panic!("expected DivisionByZero"),
        }
    }

    #[test]
    fn test_integer_arbitrary_precision() {
        // Dividend = 2^504 (64 bytes: byte 0 = 0x01, followed by 63 zero bytes)
        let mut div_a = [0x00u8; 64];
        div_a[0] = 0x01;

        // Divisor = 2^256 (33 bytes: byte 0 = 0x01, followed by 32 zero bytes)
        let mut div_b = [0x00u8; 33];
        div_b[0] = 0x01;

        let val_a = make_int(false, &div_a);
        let val_b = make_int(false, &div_b);

        // Quotient = 2^504 / 2^256 = 2^248 (32 bytes: byte 0 = 0x01, followed by 31 zero bytes)
        let res = dynamic_divide(&val_a, &val_b).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude().len(), 32);
                assert_eq!(owned.magnitude()[0], 0x01);
                for &byte in &owned.magnitude()[1..32] {
                    assert_eq!(byte, 0x00);
                }
            }
            _ => panic!("expected Integer"),
        }
    }

    #[test]
    fn test_float32_cases() {
        // finite / finite
        let res = dynamic_divide(&DynamicValue::Float32(7.0), &DynamicValue::Float32(2.0)).unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => assert_eq!(v, 3.5),
            _ => panic!("expected Float32"),
        }

        // positive / negative
        let res =
            dynamic_divide(&DynamicValue::Float32(7.0), &DynamicValue::Float32(-2.0)).unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => assert_eq!(v, -3.5),
            _ => panic!("expected Float32"),
        }

        // +0.0 / finite
        let res = dynamic_divide(&DynamicValue::Float32(0.0), &DynamicValue::Float32(5.0)).unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (0.0f32 / 5.0f32).to_bits());
            }
            _ => panic!("expected Float32"),
        }

        // -0.0 / finite
        let res =
            dynamic_divide(&DynamicValue::Float32(-0.0), &DynamicValue::Float32(5.0)).unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (-0.0f32 / 5.0f32).to_bits());
            }
            _ => panic!("expected Float32"),
        }

        // Infinity / finite
        let res = dynamic_divide(
            &DynamicValue::Float32(f32::INFINITY),
            &DynamicValue::Float32(2.0),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (f32::INFINITY / 2.0f32).to_bits());
            }
            _ => panic!("expected Float32"),
        }

        // finite / Infinity
        let res = dynamic_divide(
            &DynamicValue::Float32(2.0),
            &DynamicValue::Float32(f32::INFINITY),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (2.0f32 / f32::INFINITY).to_bits());
            }
            _ => panic!("expected Float32"),
        }

        // Infinity / Infinity -> NaN
        let res = dynamic_divide(
            &DynamicValue::Float32(f32::INFINITY),
            &DynamicValue::Float32(f32::INFINITY),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert!(v.is_nan());
            }
            _ => panic!("expected Float32"),
        }

        // NaN / finite -> NaN
        let res = dynamic_divide(
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

        // Float / zero cases:
        // 1.0 / +0.0 -> +Infinity
        let res = dynamic_divide(&DynamicValue::Float32(1.0), &DynamicValue::Float32(0.0)).unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (1.0f32 / 0.0f32).to_bits());
            }
            _ => panic!("expected Float32"),
        }

        // 1.0 / -0.0 -> -Infinity
        let res =
            dynamic_divide(&DynamicValue::Float32(1.0), &DynamicValue::Float32(-0.0)).unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (1.0f32 / -0.0f32).to_bits());
            }
            _ => panic!("expected Float32"),
        }

        // -1.0 / +0.0 -> -Infinity
        let res =
            dynamic_divide(&DynamicValue::Float32(-1.0), &DynamicValue::Float32(0.0)).unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (-1.0f32 / 0.0f32).to_bits());
            }
            _ => panic!("expected Float32"),
        }

        // -1.0 / -0.0 -> +Infinity
        let res =
            dynamic_divide(&DynamicValue::Float32(-1.0), &DynamicValue::Float32(-0.0)).unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (-1.0f32 / -0.0f32).to_bits());
            }
            _ => panic!("expected Float32"),
        }

        // 0.0 / 0.0 -> NaN
        let res = dynamic_divide(&DynamicValue::Float32(0.0), &DynamicValue::Float32(0.0)).unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert!(v.is_nan());
            }
            _ => panic!("expected Float32"),
        }
    }

    #[test]
    fn test_float64_cases() {
        // finite / finite
        let res = dynamic_divide(&DynamicValue::Float64(7.0), &DynamicValue::Float64(2.0)).unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => assert_eq!(v, 3.5),
            _ => panic!("expected Float64"),
        }

        // positive / negative
        let res =
            dynamic_divide(&DynamicValue::Float64(7.0), &DynamicValue::Float64(-2.0)).unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => assert_eq!(v, -3.5),
            _ => panic!("expected Float64"),
        }

        // +0.0 / finite
        let res = dynamic_divide(&DynamicValue::Float64(0.0), &DynamicValue::Float64(5.0)).unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert_eq!(v.to_bits(), (0.0f64 / 5.0f64).to_bits());
            }
            _ => panic!("expected Float64"),
        }

        // -0.0 / finite
        let res =
            dynamic_divide(&DynamicValue::Float64(-0.0), &DynamicValue::Float64(5.0)).unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert_eq!(v.to_bits(), (-0.0f64 / 5.0f64).to_bits());
            }
            _ => panic!("expected Float64"),
        }

        // Infinity / finite
        let res = dynamic_divide(
            &DynamicValue::Float64(f64::INFINITY),
            &DynamicValue::Float64(2.0),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert_eq!(v.to_bits(), (f64::INFINITY / 2.0f64).to_bits());
            }
            _ => panic!("expected Float64"),
        }

        // finite / Infinity
        let res = dynamic_divide(
            &DynamicValue::Float64(2.0),
            &DynamicValue::Float64(f64::INFINITY),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert_eq!(v.to_bits(), (2.0f64 / f64::INFINITY).to_bits());
            }
            _ => panic!("expected Float64"),
        }

        // Infinity / Infinity -> NaN
        let res = dynamic_divide(
            &DynamicValue::Float64(f64::INFINITY),
            &DynamicValue::Float64(f64::INFINITY),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert!(v.is_nan());
            }
            _ => panic!("expected Float64"),
        }

        // NaN / finite -> NaN
        let res = dynamic_divide(
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

        // Float / zero cases:
        // 1.0 / +0.0 -> +Infinity
        let res = dynamic_divide(&DynamicValue::Float64(1.0), &DynamicValue::Float64(0.0)).unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert_eq!(v.to_bits(), (1.0f64 / 0.0f64).to_bits());
            }
            _ => panic!("expected Float64"),
        }

        // 1.0 / -0.0 -> -Infinity
        let res =
            dynamic_divide(&DynamicValue::Float64(1.0), &DynamicValue::Float64(-0.0)).unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert_eq!(v.to_bits(), (1.0f64 / -0.0f64).to_bits());
            }
            _ => panic!("expected Float64"),
        }

        // -1.0 / +0.0 -> -Infinity
        let res =
            dynamic_divide(&DynamicValue::Float64(-1.0), &DynamicValue::Float64(0.0)).unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert_eq!(v.to_bits(), (-1.0f64 / 0.0f64).to_bits());
            }
            _ => panic!("expected Float64"),
        }

        // -1.0 / -0.0 -> +Infinity
        let res =
            dynamic_divide(&DynamicValue::Float64(-1.0), &DynamicValue::Float64(-0.0)).unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert_eq!(v.to_bits(), (-1.0f64 / -0.0f64).to_bits());
            }
            _ => panic!("expected Float64"),
        }

        // 0.0 / 0.0 -> NaN
        let res = dynamic_divide(&DynamicValue::Float64(0.0), &DynamicValue::Float64(0.0)).unwrap();
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
        let int_zero = make_int(false, &[]);
        let f32_val = DynamicValue::Float32(2.0);
        let f32_zero = DynamicValue::Float32(0.0);
        let f64_val = DynamicValue::Float64(2.0);
        let f64_zero = DynamicValue::Float64(0.0);

        // 1. Integer / Float32 (including zero to check precedence)
        match dynamic_divide(&int_val, &f32_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }
        match dynamic_divide(&int_val, &f32_zero) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }

        // 2. Integer / Float64 (including zero to check precedence)
        match dynamic_divide(&int_val, &f64_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }
        match dynamic_divide(&int_val, &f64_zero) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }

        // 3. Float32 / Integer (including zero to check precedence)
        match dynamic_divide(&f32_val, &int_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }
        match dynamic_divide(&f32_val, &int_zero) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }

        // 4. Float32 / Float64
        match dynamic_divide(&f32_val, &f64_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }

        // 5. Float64 / Integer (including zero to check precedence)
        match dynamic_divide(&f64_val, &int_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }
        match dynamic_divide(&f64_val, &int_zero) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }

        // 6. Float64 / Float32
        match dynamic_divide(&f64_val, &f32_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }
    }

    #[test]
    fn test_function_pointer_constant() {
        let op: DynamicDivide = DYNAMIC_DIVIDE;

        // 1 Integer success
        let a = make_int(false, &[42]);
        let b = make_int(false, &[6]);
        let res_int = op(&a, &b).unwrap();
        match res_int {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[7]);
            }
            _ => panic!("expected Integer"),
        }

        // 1 Integer DivisionByZero
        let zero = make_int(false, &[]);
        match op(&a, &zero) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DivisionByZero),
            _ => panic!("expected DivisionByZero"),
        }

        // 1 Float / 0.0 success
        let f1 = DynamicValue::Float32(1.0);
        let f_zero = DynamicValue::Float32(0.0);
        let res_float = op(&f1, &f_zero).unwrap();
        match res_float {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), f32::INFINITY.to_bits());
            }
            _ => panic!("expected Float32"),
        }

        // 1 DifferentFamily
        match op(&a, &f1) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }
    }
}
