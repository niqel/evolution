extern crate alloc;

use alloc::borrow::Cow;
use evo_values::dynamic_numeric::{
    DYNAMIC_ADD, DYNAMIC_DIVIDE, DYNAMIC_MULTIPLY, DYNAMIC_NEGATE, DYNAMIC_REMAINDER,
    DYNAMIC_SUBTRACT, DynamicAdd, DynamicDivide, DynamicMultiply, DynamicNegate, DynamicRemainder,
    DynamicSubtract, dynamic_add, dynamic_divide, dynamic_multiply, dynamic_negate,
    dynamic_remainder, dynamic_subtract,
};
use evo_values::{DynamicIntegerValue, DynamicNumericFailure, DynamicValue, OwnedDynamicValue};

fn make_int(negative: bool, magnitude: &[u8]) -> DynamicValue<'_> {
    DynamicValue::Integer(DynamicIntegerValue::from_parts(
        negative,
        Cow::Borrowed(magnitude),
    ))
}

// ============================================================================
// 1. Function-Pointer Surface (6/6 Use Cases via closed constants)
// ============================================================================

#[test]
fn test_function_pointer_surface_all_six_ops() {
    let negate: DynamicNegate = DYNAMIC_NEGATE;
    let add: DynamicAdd = DYNAMIC_ADD;
    let subtract: DynamicSubtract = DYNAMIC_SUBTRACT;
    let multiply: DynamicMultiply = DYNAMIC_MULTIPLY;
    let divide: DynamicDivide = DYNAMIC_DIVIDE;
    let remainder: DynamicRemainder = DYNAMIC_REMAINDER;

    let v10 = make_int(false, &[10]);
    let v3 = make_int(false, &[3]);
    let f10 = DynamicValue::Float32(10.0);
    let f3 = DynamicValue::Float32(3.0);

    // 1. Negate
    let neg_res = negate(&v10);
    match neg_res {
        OwnedDynamicValue::Integer(owned) => {
            assert!(owned.negative());
            assert_eq!(owned.magnitude(), &[10]);
        }
        _ => panic!("expected Integer"),
    }

    // 2. Add
    let add_res = add(&v10, &v3).unwrap();
    match add_res {
        OwnedDynamicValue::Integer(owned) => {
            assert!(!owned.negative());
            assert_eq!(owned.magnitude(), &[13]);
        }
        _ => panic!("expected Integer"),
    }

    // 3. Subtract
    let sub_res = subtract(&v10, &v3).unwrap();
    match sub_res {
        OwnedDynamicValue::Integer(owned) => {
            assert!(!owned.negative());
            assert_eq!(owned.magnitude(), &[7]);
        }
        _ => panic!("expected Integer"),
    }

    // 4. Multiply
    let mul_res = multiply(&v10, &v3).unwrap();
    match mul_res {
        OwnedDynamicValue::Integer(owned) => {
            assert!(!owned.negative());
            assert_eq!(owned.magnitude(), &[30]);
        }
        _ => panic!("expected Integer"),
    }

    // 5. Divide
    let div_res = divide(&v10, &v3).unwrap();
    match div_res {
        OwnedDynamicValue::Integer(owned) => {
            assert!(!owned.negative());
            assert_eq!(owned.magnitude(), &[3]);
        }
        _ => panic!("expected Integer"),
    }

    // 6. Remainder
    let rem_res = remainder(&v10, &v3).unwrap();
    match rem_res {
        OwnedDynamicValue::Integer(owned) => {
            assert!(!owned.negative());
            assert_eq!(owned.magnitude(), &[1]);
        }
        _ => panic!("expected Integer"),
    }

    // Float executions through function pointers
    match add(&f10, &f3).unwrap() {
        OwnedDynamicValue::Float32(v) => assert_eq!(v, 13.0),
        _ => panic!("expected Float32"),
    }
}

// ============================================================================
// 2. Integer Semantic Pipeline & Owned -> Borrowed Chaining
// ============================================================================

#[test]
fn test_integer_semantic_pipeline_and_owned_to_borrowed() {
    // Pipeline demonstrating:
    // a = 17, b = 5
    // Add:       17 + 5 = 22
    // Subtract:  22 - 5 = 17
    // Multiply:  17 * 5 = 85
    // Divide:    85 / 5 = 17
    // Remainder: 17 % 5 = 2
    // Negate:    17 -> -17
    let a = make_int(false, &[17]);
    let b = make_int(false, &[5]);

    // 1. Add: 17 + 5 = 22
    let step1_owned = dynamic_add(&a, &b).unwrap();
    match &step1_owned {
        OwnedDynamicValue::Integer(owned) => {
            assert!(!owned.negative());
            assert_eq!(owned.magnitude(), &[22]);
        }
        _ => panic!("expected Integer"),
    }

    // 2. Subtract: 22 - 5 = 17 (consuming step1 via as_borrowed)
    let step1_borrowed = step1_owned.as_borrowed();
    let step2_owned = dynamic_subtract(&step1_borrowed, &b).unwrap();
    match &step2_owned {
        OwnedDynamicValue::Integer(owned) => {
            assert!(!owned.negative());
            assert_eq!(owned.magnitude(), &[17]);
        }
        _ => panic!("expected Integer"),
    }

    // 3. Multiply: 17 * 5 = 85 (consuming step2 via as_borrowed)
    let step2_borrowed = step2_owned.as_borrowed();
    let step3_owned = dynamic_multiply(&step2_borrowed, &b).unwrap();
    match &step3_owned {
        OwnedDynamicValue::Integer(owned) => {
            assert!(!owned.negative());
            assert_eq!(owned.magnitude(), &[85]);
        }
        _ => panic!("expected Integer"),
    }

    // 4. Divide: 85 / 5 = 17 (consuming step3 via as_borrowed)
    let step3_borrowed = step3_owned.as_borrowed();
    let step4_owned = dynamic_divide(&step3_borrowed, &b).unwrap();
    match &step4_owned {
        OwnedDynamicValue::Integer(owned) => {
            assert!(!owned.negative());
            assert_eq!(owned.magnitude(), &[17]);
        }
        _ => panic!("expected Integer"),
    }

    // 5. Remainder: 17 % 5 = 2 (consuming step4 via as_borrowed)
    let step4_borrowed = step4_owned.as_borrowed();
    let step5_owned = dynamic_remainder(&step4_borrowed, &b).unwrap();
    match &step5_owned {
        OwnedDynamicValue::Integer(owned) => {
            assert!(!owned.negative());
            assert_eq!(owned.magnitude(), &[2]);
        }
        _ => panic!("expected Integer"),
    }

    // 6. Negate: 17 -> -17 (consuming step4 via as_borrowed)
    let step6_owned = dynamic_negate(&step4_borrowed);
    match &step6_owned {
        OwnedDynamicValue::Integer(owned) => {
            assert!(owned.negative());
            assert_eq!(owned.magnitude(), &[17]);
        }
        _ => panic!("expected Integer"),
    }
}

// ============================================================================
// 3. Integer Canonical Zero
// ============================================================================

#[test]
fn test_integer_canonical_zero_across_operations() {
    let n = make_int(false, &[42]);
    let neg_n = make_int(true, &[42]);
    let zero = make_int(false, &[]);

    // N + (-N) = 0
    let res_add = dynamic_add(&n, &neg_n).unwrap();
    match res_add {
        OwnedDynamicValue::Integer(owned) => {
            assert!(!owned.negative());
            assert_eq!(owned.magnitude(), &[]);
        }
        _ => panic!("expected Integer"),
    }

    // N - N = 0
    let res_sub = dynamic_subtract(&n, &n).unwrap();
    match res_sub {
        OwnedDynamicValue::Integer(owned) => {
            assert!(!owned.negative());
            assert_eq!(owned.magnitude(), &[]);
        }
        _ => panic!("expected Integer"),
    }

    // N * 0 = 0
    let res_mul = dynamic_multiply(&n, &zero).unwrap();
    match res_mul {
        OwnedDynamicValue::Integer(owned) => {
            assert!(!owned.negative());
            assert_eq!(owned.magnitude(), &[]);
        }
        _ => panic!("expected Integer"),
    }

    // 0 * N = 0
    let res_mul_zero = dynamic_multiply(&zero, &n).unwrap();
    match res_mul_zero {
        OwnedDynamicValue::Integer(owned) => {
            assert!(!owned.negative());
            assert_eq!(owned.magnitude(), &[]);
        }
        _ => panic!("expected Integer"),
    }

    // N % N = 0
    let res_rem = dynamic_remainder(&n, &n).unwrap();
    match res_rem {
        OwnedDynamicValue::Integer(owned) => {
            assert!(!owned.negative());
            assert_eq!(owned.magnitude(), &[]);
        }
        _ => panic!("expected Integer"),
    }

    // -N % N = 0
    let res_rem_neg = dynamic_remainder(&neg_n, &n).unwrap();
    match res_rem_neg {
        OwnedDynamicValue::Integer(owned) => {
            assert!(!owned.negative());
            assert_eq!(owned.magnitude(), &[]);
        }
        _ => panic!("expected Integer"),
    }
}

// ============================================================================
// 4. Integer Arbitrary Precision (> i128)
// ============================================================================

#[test]
fn test_integer_arbitrary_precision_pipeline() {
    // Operands clearly > i128 (16 bytes). We use 256-bit numbers (33 bytes with leading bit).
    // x = 2^256 (33 bytes: [0x01, 0x00, ... 32 zeros])
    let mut mag_x = [0x00u8; 33];
    mag_x[0] = 0x01;
    let x = make_int(false, &mag_x);

    // two = 2
    let two = make_int(false, &[2]);
    // four = 4
    let four = make_int(false, &[4]);
    // three = 3
    let three = make_int(false, &[3]);

    // 1. Add: x + x = 2 * 2^256 = 2^257 (33 bytes: [0x02, 0x00, ... 32 zeros])
    let step_add = dynamic_add(&x, &x).unwrap();
    match &step_add {
        OwnedDynamicValue::Integer(owned) => {
            assert!(!owned.negative());
            assert_eq!(owned.magnitude().len(), 33);
            assert_eq!(owned.magnitude()[0], 0x02);
            for &b in &owned.magnitude()[1..] {
                assert_eq!(b, 0x00);
            }
        }
        _ => panic!("expected Integer"),
    }

    // 2. Multiply: (2^257) * 2 = 2^258 (33 bytes: [0x04, 0x00, ... 32 zeros])
    let step_mul = dynamic_multiply(&step_add.as_borrowed(), &two).unwrap();
    match &step_mul {
        OwnedDynamicValue::Integer(owned) => {
            assert!(!owned.negative());
            assert_eq!(owned.magnitude().len(), 33);
            assert_eq!(owned.magnitude()[0], 0x04);
            for &b in &owned.magnitude()[1..] {
                assert_eq!(b, 0x00);
            }
        }
        _ => panic!("expected Integer"),
    }

    // 3. Divide: (2^258) / 4 = 2^256 (33 bytes: [0x01, 0x00, ... 32 zeros]) = x
    let step_div = dynamic_divide(&step_mul.as_borrowed(), &four).unwrap();
    match &step_div {
        OwnedDynamicValue::Integer(owned) => {
            assert!(!owned.negative());
            assert_eq!(owned.magnitude().len(), 33);
            assert_eq!(owned.magnitude()[0], 0x01);
            for &b in &owned.magnitude()[1..] {
                assert_eq!(b, 0x00);
            }
        }
        _ => panic!("expected Integer"),
    }

    // 4. Subtract: step_div - x = 0 (canonical zero!)
    let step_sub = dynamic_subtract(&step_div.as_borrowed(), &x).unwrap();
    match &step_sub {
        OwnedDynamicValue::Integer(owned) => {
            assert!(!owned.negative());
            assert_eq!(owned.magnitude(), &[]);
        }
        _ => panic!("expected Integer"),
    }

    // 5. Remainder: 2^256 % 3 = 1 (since 2 = -1 mod 3, 2^256 = (-1)^256 = 1 mod 3)
    let step_rem = dynamic_remainder(&x, &three).unwrap();
    match &step_rem {
        OwnedDynamicValue::Integer(owned) => {
            assert!(!owned.negative());
            assert_eq!(owned.magnitude(), &[1]);
        }
        _ => panic!("expected Integer"),
    }
}

// ============================================================================
// 5. Integer Division & Remainder Identity (a = q * b + r)
// ============================================================================

#[test]
fn test_integer_division_and_remainder_identity_all_signs() {
    let cases = [
        (make_int(false, &[17]), make_int(false, &[5])), // positive / positive
        (make_int(true, &[17]), make_int(false, &[5])),  // negative / positive
        (make_int(false, &[17]), make_int(true, &[5])),  // positive / negative
        (make_int(true, &[17]), make_int(true, &[5])),   // negative / negative
    ];

    for (a, b) in &cases {
        let q_owned = dynamic_divide(a, b).unwrap();
        let r_owned = dynamic_remainder(a, b).unwrap();

        // Check quotient and remainder values explicitly
        match (a, b, &q_owned, &r_owned) {
            (
                DynamicValue::Integer(a_val),
                DynamicValue::Integer(b_val),
                OwnedDynamicValue::Integer(q_val),
                OwnedDynamicValue::Integer(r_val),
            ) => {
                let a_neg = a_val.negative();
                let b_neg = b_val.negative();
                let expected_q_neg = a_neg != b_neg;
                let expected_r_neg = a_neg;

                assert_eq!(q_val.negative(), expected_q_neg);
                assert_eq!(q_val.magnitude(), &[3]); // 17 / 5 = 3 (truncation toward zero)

                assert_eq!(r_val.negative(), expected_r_neg);
                assert_eq!(r_val.magnitude(), &[2]); // 17 % 5 = 2
            }
            _ => panic!("expected Integer values"),
        }

        // Mathematical identity: a == q * b + r
        let q_times_b = dynamic_multiply(&q_owned.as_borrowed(), b).unwrap();
        let reconstructed = dynamic_add(&q_times_b.as_borrowed(), &r_owned.as_borrowed()).unwrap();

        match (a, &reconstructed) {
            (DynamicValue::Integer(expected), OwnedDynamicValue::Integer(actual)) => {
                assert_eq!(expected.negative(), actual.negative());
                assert_eq!(expected.magnitude(), actual.magnitude());
            }
            _ => panic!("expected Integers"),
        }
    }
}

fn assert_failure<T>(res: Result<T, DynamicNumericFailure>, expected: DynamicNumericFailure) {
    match res {
        Err(err) => assert_eq!(err, expected),
        Ok(_) => panic!("expected failure"),
    }
}

// ============================================================================
// 6. Failures — DivisionByZero
// ============================================================================

#[test]
fn test_failure_division_by_zero() {
    let pos_7 = make_int(false, &[7]);
    let neg_7 = make_int(true, &[7]);
    let zero = make_int(false, &[]);

    // Integer / 0
    assert_failure(
        dynamic_divide(&pos_7, &zero),
        DynamicNumericFailure::DivisionByZero,
    );
    assert_failure(
        dynamic_divide(&neg_7, &zero),
        DynamicNumericFailure::DivisionByZero,
    );
    assert_failure(
        dynamic_divide(&zero, &zero),
        DynamicNumericFailure::DivisionByZero,
    );

    // Integer % 0
    assert_failure(
        dynamic_remainder(&pos_7, &zero),
        DynamicNumericFailure::DivisionByZero,
    );
    assert_failure(
        dynamic_remainder(&neg_7, &zero),
        DynamicNumericFailure::DivisionByZero,
    );
    assert_failure(
        dynamic_remainder(&zero, &zero),
        DynamicNumericFailure::DivisionByZero,
    );

    // Verify Add, Subtract, Multiply with zero do NOT produce DivisionByZero
    assert!(dynamic_add(&pos_7, &zero).is_ok());
    assert!(dynamic_subtract(&pos_7, &zero).is_ok());
    assert!(dynamic_multiply(&pos_7, &zero).is_ok());
}

// ============================================================================
// 7. Failures — DifferentFamily Precedence
// ============================================================================

#[test]
fn test_failure_different_family_all_directions_and_zero_precedence() {
    let int_val = make_int(false, &[10]);
    let int_zero = make_int(false, &[]);
    let f32_val = DynamicValue::Float32(10.0);
    let f32_zero = DynamicValue::Float32(0.0);
    let f64_val = DynamicValue::Float64(10.0);
    let f64_zero = DynamicValue::Float64(0.0);

    let binary_ops = [
        dynamic_add,
        dynamic_subtract,
        dynamic_multiply,
        dynamic_divide,
        dynamic_remainder,
    ];

    for op in binary_ops {
        // 1. Integer ↔ Float32
        assert_failure(
            op(&int_val, &f32_val),
            DynamicNumericFailure::DifferentFamily,
        );
        assert_failure(
            op(&f32_val, &int_val),
            DynamicNumericFailure::DifferentFamily,
        );

        // 2. Integer ↔ Float64
        assert_failure(
            op(&int_val, &f64_val),
            DynamicNumericFailure::DifferentFamily,
        );
        assert_failure(
            op(&f64_val, &int_val),
            DynamicNumericFailure::DifferentFamily,
        );

        // 3. Float32 ↔ Float64
        assert_failure(
            op(&f32_val, &f64_val),
            DynamicNumericFailure::DifferentFamily,
        );
        assert_failure(
            op(&f64_val, &f32_val),
            DynamicNumericFailure::DifferentFamily,
        );

        // Zero precedence test: DifferentFamily must precede DivisionByZero
        assert_failure(
            op(&int_val, &f32_zero),
            DynamicNumericFailure::DifferentFamily,
        );
        assert_failure(
            op(&int_zero, &f32_zero),
            DynamicNumericFailure::DifferentFamily,
        );
        assert_failure(
            op(&f64_val, &int_zero),
            DynamicNumericFailure::DifferentFamily,
        );
        assert_failure(
            op(&f64_zero, &int_zero),
            DynamicNumericFailure::DifferentFamily,
        );
    }
}

// ============================================================================
// 8. Float32 Matrix (All 6 Use Cases)
// ============================================================================

#[test]
fn test_float32_matrix_all_six_ops() {
    let f_pos = DynamicValue::Float32(5.5);
    let f_neg = DynamicValue::Float32(-2.0);
    let f_zero_pos = DynamicValue::Float32(0.0);
    let f_zero_neg = DynamicValue::Float32(-0.0);
    let f_inf = DynamicValue::Float32(f32::INFINITY);
    let f_nan = DynamicValue::Float32(f32::NAN);
    let f_max = DynamicValue::Float32(f32::MAX);

    // 1. Negate
    match dynamic_negate(&f_zero_pos) {
        OwnedDynamicValue::Float32(v) => assert_eq!(v.to_bits(), (-0.0f32).to_bits()),
        _ => panic!("expected Float32"),
    }
    match dynamic_negate(&f_zero_neg) {
        OwnedDynamicValue::Float32(v) => assert_eq!(v.to_bits(), 0.0f32.to_bits()),
        _ => panic!("expected Float32"),
    }

    // 2. Add (including IEEE overflow to Infinity)
    match dynamic_add(&f_max, &f_max).unwrap() {
        OwnedDynamicValue::Float32(v) => assert_eq!(v.to_bits(), f32::INFINITY.to_bits()),
        _ => panic!("expected Float32"),
    }

    // 3. Subtract (finite - finite)
    match dynamic_subtract(&f_pos, &f_neg).unwrap() {
        OwnedDynamicValue::Float32(v) => assert_eq!(v, 7.5),
        _ => panic!("expected Float32"),
    }

    // 4. Multiply (Infinity * 0.0 -> NaN)
    match dynamic_multiply(&f_inf, &f_zero_pos).unwrap() {
        OwnedDynamicValue::Float32(v) => assert!(v.is_nan()),
        _ => panic!("expected Float32"),
    }

    // 5. Divide (finite / ±0.0 -> ±Infinity, 0.0 / 0.0 -> NaN)
    match dynamic_divide(&f_pos, &f_zero_pos).unwrap() {
        OwnedDynamicValue::Float32(v) => assert_eq!(v.to_bits(), f32::INFINITY.to_bits()),
        _ => panic!("expected Float32"),
    }
    match dynamic_divide(&f_pos, &f_zero_neg).unwrap() {
        OwnedDynamicValue::Float32(v) => assert_eq!(v.to_bits(), f32::NEG_INFINITY.to_bits()),
        _ => panic!("expected Float32"),
    }
    match dynamic_divide(&f_zero_pos, &f_zero_pos).unwrap() {
        OwnedDynamicValue::Float32(v) => assert!(v.is_nan()),
        _ => panic!("expected Float32"),
    }

    // 6. Remainder (finite % finite, finite % ±0.0 -> NaN)
    match dynamic_remainder(&DynamicValue::Float32(7.5), &DynamicValue::Float32(2.0)).unwrap() {
        OwnedDynamicValue::Float32(v) => assert_eq!(v.to_bits(), (7.5f32 % 2.0f32).to_bits()),
        _ => panic!("expected Float32"),
    }
    match dynamic_remainder(&f_pos, &f_zero_pos).unwrap() {
        OwnedDynamicValue::Float32(v) => assert!(v.is_nan()),
        _ => panic!("expected Float32"),
    }
    match dynamic_remainder(&f_nan, &f_pos).unwrap() {
        OwnedDynamicValue::Float32(v) => assert!(v.is_nan()),
        _ => panic!("expected Float32"),
    }
}

// ============================================================================
// 9. Float64 Matrix (All 6 Use Cases)
// ============================================================================

#[test]
fn test_float64_matrix_all_six_ops() {
    let f_pos = DynamicValue::Float64(5.5);
    let f_neg = DynamicValue::Float64(-2.0);
    let f_zero_pos = DynamicValue::Float64(0.0);
    let f_zero_neg = DynamicValue::Float64(-0.0);
    let f_inf = DynamicValue::Float64(f64::INFINITY);
    let f_nan = DynamicValue::Float64(f64::NAN);
    let f_max = DynamicValue::Float64(f64::MAX);

    // 1. Negate
    match dynamic_negate(&f_zero_pos) {
        OwnedDynamicValue::Float64(v) => assert_eq!(v.to_bits(), (-0.0f64).to_bits()),
        _ => panic!("expected Float64"),
    }
    match dynamic_negate(&f_zero_neg) {
        OwnedDynamicValue::Float64(v) => assert_eq!(v.to_bits(), 0.0f64.to_bits()),
        _ => panic!("expected Float64"),
    }

    // 2. Add (including IEEE overflow to Infinity)
    match dynamic_add(&f_max, &f_max).unwrap() {
        OwnedDynamicValue::Float64(v) => assert_eq!(v.to_bits(), f64::INFINITY.to_bits()),
        _ => panic!("expected Float64"),
    }

    // 3. Subtract (finite - finite)
    match dynamic_subtract(&f_pos, &f_neg).unwrap() {
        OwnedDynamicValue::Float64(v) => assert_eq!(v, 7.5),
        _ => panic!("expected Float64"),
    }

    // 4. Multiply (Infinity * 0.0 -> NaN)
    match dynamic_multiply(&f_inf, &f_zero_pos).unwrap() {
        OwnedDynamicValue::Float64(v) => assert!(v.is_nan()),
        _ => panic!("expected Float64"),
    }

    // 5. Divide (finite / ±0.0 -> ±Infinity, 0.0 / 0.0 -> NaN)
    match dynamic_divide(&f_pos, &f_zero_pos).unwrap() {
        OwnedDynamicValue::Float64(v) => assert_eq!(v.to_bits(), f64::INFINITY.to_bits()),
        _ => panic!("expected Float64"),
    }
    match dynamic_divide(&f_pos, &f_zero_neg).unwrap() {
        OwnedDynamicValue::Float64(v) => assert_eq!(v.to_bits(), f64::NEG_INFINITY.to_bits()),
        _ => panic!("expected Float64"),
    }
    match dynamic_divide(&f_zero_pos, &f_zero_pos).unwrap() {
        OwnedDynamicValue::Float64(v) => assert!(v.is_nan()),
        _ => panic!("expected Float64"),
    }

    // 6. Remainder (finite % finite, finite % ±0.0 -> NaN)
    match dynamic_remainder(&DynamicValue::Float64(7.5), &DynamicValue::Float64(2.0)).unwrap() {
        OwnedDynamicValue::Float64(v) => assert_eq!(v.to_bits(), (7.5f64 % 2.0f64).to_bits()),
        _ => panic!("expected Float64"),
    }
    match dynamic_remainder(&f_pos, &f_zero_pos).unwrap() {
        OwnedDynamicValue::Float64(v) => assert!(v.is_nan()),
        _ => panic!("expected Float64"),
    }
    match dynamic_remainder(&f_nan, &f_pos).unwrap() {
        OwnedDynamicValue::Float64(v) => assert!(v.is_nan()),
        _ => panic!("expected Float64"),
    }
}

// ============================================================================
// 10. Conceptual Matrix (6 Use Cases x 3 Families Full Cross Verification)
// ============================================================================

#[test]
fn test_conceptual_matrix_6x3_full_coverage() {
    let int_a = make_int(false, &[8]);
    let int_b = make_int(false, &[2]);

    let f32_a = DynamicValue::Float32(8.0);
    let f32_b = DynamicValue::Float32(2.0);

    let f64_a = DynamicValue::Float64(8.0);
    let f64_b = DynamicValue::Float64(2.0);

    // --- Negate ---
    assert!(matches!(
        dynamic_negate(&int_a),
        OwnedDynamicValue::Integer(_)
    ));
    assert!(matches!(
        dynamic_negate(&f32_a),
        OwnedDynamicValue::Float32(_)
    ));
    assert!(matches!(
        dynamic_negate(&f64_a),
        OwnedDynamicValue::Float64(_)
    ));

    // --- Add ---
    assert!(matches!(
        dynamic_add(&int_a, &int_b).unwrap(),
        OwnedDynamicValue::Integer(_)
    ));
    assert!(matches!(
        dynamic_add(&f32_a, &f32_b).unwrap(),
        OwnedDynamicValue::Float32(_)
    ));
    assert!(matches!(
        dynamic_add(&f64_a, &f64_b).unwrap(),
        OwnedDynamicValue::Float64(_)
    ));

    // --- Subtract ---
    assert!(matches!(
        dynamic_subtract(&int_a, &int_b).unwrap(),
        OwnedDynamicValue::Integer(_)
    ));
    assert!(matches!(
        dynamic_subtract(&f32_a, &f32_b).unwrap(),
        OwnedDynamicValue::Float32(_)
    ));
    assert!(matches!(
        dynamic_subtract(&f64_a, &f64_b).unwrap(),
        OwnedDynamicValue::Float64(_)
    ));

    // --- Multiply ---
    assert!(matches!(
        dynamic_multiply(&int_a, &int_b).unwrap(),
        OwnedDynamicValue::Integer(_)
    ));
    assert!(matches!(
        dynamic_multiply(&f32_a, &f32_b).unwrap(),
        OwnedDynamicValue::Float32(_)
    ));
    assert!(matches!(
        dynamic_multiply(&f64_a, &f64_b).unwrap(),
        OwnedDynamicValue::Float64(_)
    ));

    // --- Divide ---
    assert!(matches!(
        dynamic_divide(&int_a, &int_b).unwrap(),
        OwnedDynamicValue::Integer(_)
    ));
    assert!(matches!(
        dynamic_divide(&f32_a, &f32_b).unwrap(),
        OwnedDynamicValue::Float32(_)
    ));
    assert!(matches!(
        dynamic_divide(&f64_a, &f64_b).unwrap(),
        OwnedDynamicValue::Float64(_)
    ));

    // --- Remainder ---
    assert!(matches!(
        dynamic_remainder(&int_a, &int_b).unwrap(),
        OwnedDynamicValue::Integer(_)
    ));
    assert!(matches!(
        dynamic_remainder(&f32_a, &f32_b).unwrap(),
        OwnedDynamicValue::Float32(_)
    ));
    assert!(matches!(
        dynamic_remainder(&f64_a, &f64_b).unwrap(),
        OwnedDynamicValue::Float64(_)
    ));
}
