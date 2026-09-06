use super::integer::{from_big_int, to_big_int};
use crate::definitions::dynamic_numeric::DynamicRemainder;
use crate::definitions::failures::DynamicNumericFailure;
use crate::definitions::value::{DynamicValue, OwnedDynamicValue};

pub fn dynamic_remainder<'left, 'right>(
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
            let rem = left_big % right_big;
            Ok(OwnedDynamicValue::Integer(from_big_int(rem)))
        }
        (DynamicValue::Float32(l), DynamicValue::Float32(r)) => {
            Ok(OwnedDynamicValue::Float32(l % r))
        }
        (DynamicValue::Float64(l), DynamicValue::Float64(r)) => {
            Ok(OwnedDynamicValue::Float64(l % r))
        }
        _ => Err(DynamicNumericFailure::DifferentFamily),
    }
}

pub const DYNAMIC_REMAINDER: DynamicRemainder = dynamic_remainder;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::value::DynamicIntegerValue;
    use crate::dynamic_numeric::divide::dynamic_divide;
    use crate::dynamic_numeric::multiply::dynamic_multiply;
    use alloc::borrow::Cow;

    fn make_int(negative: bool, magnitude: &[u8]) -> DynamicValue<'_> {
        DynamicValue::Integer(DynamicIntegerValue::from_parts(
            negative,
            Cow::Borrowed(magnitude),
        ))
    }

    #[test]
    fn test_integer_signs() {
        let pos_7 = make_int(false, &[7]);
        let pos_3 = make_int(false, &[3]);
        let neg_7 = make_int(true, &[7]);
        let neg_3 = make_int(true, &[3]);

        //  7 %  3 =  1
        let res = dynamic_remainder(&pos_7, &pos_3).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[1]);
            }
            _ => panic!("expected Integer"),
        }

        // -7 %  3 = -1
        let res = dynamic_remainder(&neg_7, &pos_3).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(owned.negative());
                assert_eq!(owned.magnitude(), &[1]);
            }
            _ => panic!("expected Integer"),
        }

        //  7 % -3 =  1
        let res = dynamic_remainder(&pos_7, &neg_3).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[1]);
            }
            _ => panic!("expected Integer"),
        }

        // -7 % -3 = -1
        let res = dynamic_remainder(&neg_7, &neg_3).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(owned.negative());
                assert_eq!(owned.magnitude(), &[1]);
            }
            _ => panic!("expected Integer"),
        }
    }

    #[test]
    fn test_integer_canonical_zero() {
        let pos_6 = make_int(false, &[6]);
        let pos_3 = make_int(false, &[3]);
        let neg_6 = make_int(true, &[6]);
        let neg_3 = make_int(true, &[3]);

        //  6 %  3 = 0
        let res = dynamic_remainder(&pos_6, &pos_3).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[]);
            }
            _ => panic!("expected Integer"),
        }

        // -6 %  3 = 0
        let res = dynamic_remainder(&neg_6, &pos_3).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[]);
            }
            _ => panic!("expected Integer"),
        }

        //  6 % -3 = 0
        let res = dynamic_remainder(&pos_6, &neg_3).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[]);
            }
            _ => panic!("expected Integer"),
        }

        // -6 % -3 = 0
        let res = dynamic_remainder(&neg_6, &neg_3).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[]);
            }
            _ => panic!("expected Integer"),
        }
    }

    #[test]
    fn test_integer_identity_quotient_remainder() {
        // Test a = q * b + r across all 4 sign pairs
        let cases = [
            (make_int(false, &[17]), make_int(false, &[5])), // pos / pos
            (make_int(true, &[17]), make_int(false, &[5])),  // neg / pos
            (make_int(false, &[17]), make_int(true, &[5])),  // pos / neg
            (make_int(true, &[17]), make_int(true, &[5])),   // neg / neg
        ];

        for (a, b) in &cases {
            let q = dynamic_divide(a, b).unwrap();
            let r = dynamic_remainder(a, b).unwrap();

            // Convert OwnedDynamicValue back to DynamicValue for multiplication and addition
            let q_borrowed = q.as_borrowed();
            let q_times_b = dynamic_multiply(&q_borrowed, b).unwrap();
            let q_times_b_borrowed = q_times_b.as_borrowed();
            let r_borrowed = r.as_borrowed();
            let reconstructed =
                crate::dynamic_numeric::add::dynamic_add(&q_times_b_borrowed, &r_borrowed).unwrap();

            match (a, &reconstructed) {
                (DynamicValue::Integer(expected), OwnedDynamicValue::Integer(actual)) => {
                    assert_eq!(expected.negative(), actual.negative());
                    assert_eq!(expected.magnitude(), actual.magnitude());
                }
                _ => panic!("expected Integers"),
            }
        }
    }

    #[test]
    fn test_integer_division_by_zero() {
        let pos_7 = make_int(false, &[7]);
        let neg_7 = make_int(true, &[7]);
        let zero = make_int(false, &[]);

        // positive % 0
        match dynamic_remainder(&pos_7, &zero) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DivisionByZero),
            _ => panic!("expected DivisionByZero"),
        }

        // negative % 0
        match dynamic_remainder(&neg_7, &zero) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DivisionByZero),
            _ => panic!("expected DivisionByZero"),
        }

        // 0 % 0
        match dynamic_remainder(&zero, &zero) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DivisionByZero),
            _ => panic!("expected DivisionByZero"),
        }
    }

    #[test]
    fn test_integer_arbitrary_precision() {
        // Dividend = 2^504 + 0x1234 (64 bytes: byte 0 = 0x01, byte 62 = 0x12, byte 63 = 0x34)
        let mut div_a = [0x00u8; 64];
        div_a[0] = 0x01;
        div_a[62] = 0x12;
        div_a[63] = 0x34;

        // Divisor = 2^256 (33 bytes: byte 0 = 0x01, rest zeros)
        let mut div_b = [0x00u8; 33];
        div_b[0] = 0x01;

        let val_a = make_int(false, &div_a);
        let val_b = make_int(false, &div_b);

        // Remainder of (2^504 + 0x1234) % 2^256 is 0x1234 = [0x12, 0x34]
        let res = dynamic_remainder(&val_a, &val_b).unwrap();
        match res {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[0x12, 0x34]);
            }
            _ => panic!("expected Integer"),
        }
    }

    #[test]
    fn test_float32_cases() {
        // finite % finite
        let res =
            dynamic_remainder(&DynamicValue::Float32(7.5), &DynamicValue::Float32(2.0)).unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (7.5f32 % 2.0f32).to_bits());
            }
            _ => panic!("expected Float32"),
        }

        // positive % negative
        let res =
            dynamic_remainder(&DynamicValue::Float32(7.5), &DynamicValue::Float32(-2.0)).unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (7.5f32 % -2.0f32).to_bits());
            }
            _ => panic!("expected Float32"),
        }

        // negative % positive
        let res =
            dynamic_remainder(&DynamicValue::Float32(-7.5), &DynamicValue::Float32(2.0)).unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (-7.5f32 % 2.0f32).to_bits());
            }
            _ => panic!("expected Float32"),
        }

        // +0.0 % finite
        let res =
            dynamic_remainder(&DynamicValue::Float32(0.0), &DynamicValue::Float32(2.0)).unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (0.0f32 % 2.0f32).to_bits());
            }
            _ => panic!("expected Float32"),
        }

        // -0.0 % finite
        let res =
            dynamic_remainder(&DynamicValue::Float32(-0.0), &DynamicValue::Float32(2.0)).unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (-0.0f32 % 2.0f32).to_bits());
            }
            _ => panic!("expected Float32"),
        }

        // finite % +0.0 -> NaN
        let res =
            dynamic_remainder(&DynamicValue::Float32(5.0), &DynamicValue::Float32(0.0)).unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert!(v.is_nan());
            }
            _ => panic!("expected Float32"),
        }

        // finite % -0.0 -> NaN
        let res =
            dynamic_remainder(&DynamicValue::Float32(5.0), &DynamicValue::Float32(-0.0)).unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert!(v.is_nan());
            }
            _ => panic!("expected Float32"),
        }

        // Infinity % finite -> NaN
        let res = dynamic_remainder(
            &DynamicValue::Float32(f32::INFINITY),
            &DynamicValue::Float32(2.0),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert!(v.is_nan());
            }
            _ => panic!("expected Float32"),
        }

        // finite % Infinity -> finite
        let res = dynamic_remainder(
            &DynamicValue::Float32(2.0),
            &DynamicValue::Float32(f32::INFINITY),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (2.0f32 % f32::INFINITY).to_bits());
            }
            _ => panic!("expected Float32"),
        }

        // Infinity % Infinity -> NaN
        let res = dynamic_remainder(
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

        // NaN % finite -> NaN
        let res = dynamic_remainder(
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
        // finite % finite
        let res =
            dynamic_remainder(&DynamicValue::Float64(7.5), &DynamicValue::Float64(2.0)).unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert_eq!(v.to_bits(), (7.5f64 % 2.0f64).to_bits());
            }
            _ => panic!("expected Float64"),
        }

        // positive % negative
        let res =
            dynamic_remainder(&DynamicValue::Float64(7.5), &DynamicValue::Float64(-2.0)).unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert_eq!(v.to_bits(), (7.5f64 % -2.0f64).to_bits());
            }
            _ => panic!("expected Float64"),
        }

        // negative % positive
        let res =
            dynamic_remainder(&DynamicValue::Float64(-7.5), &DynamicValue::Float64(2.0)).unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert_eq!(v.to_bits(), (-7.5f64 % 2.0f64).to_bits());
            }
            _ => panic!("expected Float64"),
        }

        // +0.0 % finite
        let res =
            dynamic_remainder(&DynamicValue::Float64(0.0), &DynamicValue::Float64(2.0)).unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert_eq!(v.to_bits(), (0.0f64 % 2.0f64).to_bits());
            }
            _ => panic!("expected Float64"),
        }

        // -0.0 % finite
        let res =
            dynamic_remainder(&DynamicValue::Float64(-0.0), &DynamicValue::Float64(2.0)).unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert_eq!(v.to_bits(), (-0.0f64 % 2.0f64).to_bits());
            }
            _ => panic!("expected Float64"),
        }

        // finite % +0.0 -> NaN
        let res =
            dynamic_remainder(&DynamicValue::Float64(5.0), &DynamicValue::Float64(0.0)).unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert!(v.is_nan());
            }
            _ => panic!("expected Float64"),
        }

        // finite % -0.0 -> NaN
        let res =
            dynamic_remainder(&DynamicValue::Float64(5.0), &DynamicValue::Float64(-0.0)).unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert!(v.is_nan());
            }
            _ => panic!("expected Float64"),
        }

        // Infinity % finite -> NaN
        let res = dynamic_remainder(
            &DynamicValue::Float64(f64::INFINITY),
            &DynamicValue::Float64(2.0),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert!(v.is_nan());
            }
            _ => panic!("expected Float64"),
        }

        // finite % Infinity -> finite
        let res = dynamic_remainder(
            &DynamicValue::Float64(2.0),
            &DynamicValue::Float64(f64::INFINITY),
        )
        .unwrap();
        match res {
            OwnedDynamicValue::Float64(v) => {
                assert_eq!(v.to_bits(), (2.0f64 % f64::INFINITY).to_bits());
            }
            _ => panic!("expected Float64"),
        }

        // Infinity % Infinity -> NaN
        let res = dynamic_remainder(
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

        // NaN % finite -> NaN
        let res = dynamic_remainder(
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
        let int_zero = make_int(false, &[]);
        let f32_val = DynamicValue::Float32(2.0);
        let f32_zero = DynamicValue::Float32(0.0);
        let f64_val = DynamicValue::Float64(2.0);
        let f64_zero = DynamicValue::Float64(0.0);

        // 1. Integer / Float32 (including zero)
        match dynamic_remainder(&int_val, &f32_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }
        match dynamic_remainder(&int_val, &f32_zero) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }

        // 2. Integer / Float64 (including zero)
        match dynamic_remainder(&int_val, &f64_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }
        match dynamic_remainder(&int_val, &f64_zero) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }

        // 3. Float32 / Integer (including zero)
        match dynamic_remainder(&f32_val, &int_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }
        match dynamic_remainder(&f32_val, &int_zero) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }

        // 4. Float32 / Float64
        match dynamic_remainder(&f32_val, &f64_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }

        // 5. Float64 / Integer (including zero)
        match dynamic_remainder(&f64_val, &int_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }
        match dynamic_remainder(&f64_val, &int_zero) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }

        // 6. Float64 / Float32
        match dynamic_remainder(&f64_val, &f32_val) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DifferentFamily),
            _ => panic!("expected DifferentFamily"),
        }
    }

    #[test]
    fn test_function_pointer_constant() {
        let op: DynamicRemainder = DYNAMIC_REMAINDER;

        // 1 Integer success
        let a = make_int(false, &[45]);
        let b = make_int(false, &[6]);
        let res_int = op(&a, &b).unwrap();
        match res_int {
            OwnedDynamicValue::Integer(owned) => {
                assert!(!owned.negative());
                assert_eq!(owned.magnitude(), &[3]); // 45 % 6 = 3
            }
            _ => panic!("expected Integer"),
        }

        // 1 Integer DivisionByZero
        let zero = make_int(false, &[]);
        match op(&a, &zero) {
            Err(err) => assert_eq!(err, DynamicNumericFailure::DivisionByZero),
            _ => panic!("expected DivisionByZero"),
        }

        // 1 Float remainder
        let f1 = DynamicValue::Float32(5.5);
        let f2 = DynamicValue::Float32(2.0);
        let res_float = op(&f1, &f2).unwrap();
        match res_float {
            OwnedDynamicValue::Float32(v) => {
                assert_eq!(v.to_bits(), (5.5f32 % 2.0f32).to_bits());
            }
            _ => panic!("expected Float32"),
        }

        // 1 Float % 0.0 success (evaluates to NaN)
        let f_zero = DynamicValue::Float32(0.0);
        let res_f_zero = op(&f1, &f_zero).unwrap();
        match res_f_zero {
            OwnedDynamicValue::Float32(v) => {
                assert!(v.is_nan());
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
