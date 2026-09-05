use alloc::borrow::Cow;
use alloc::boxed::Box;

use super::kernel::*;
use crate::definitions::comparison::*;
use crate::definitions::failures::ComparisonFailure;
use crate::definitions::value::{DynamicIntegerValue, DynamicValue, EnumPayload, Value};

// ============================================================================
// 1. Different Family tests (Section 22)
// ============================================================================

#[test]
fn test_different_family_cases() {
    let cases: [(&Value<'_>, &Value<'_>); 8] = [
        (&Value::Boolean(true), &Value::Int8(1)),
        (&Value::Int8(1), &Value::Int16(1)),
        (&Value::Int32(10), &Value::Uint32(10)),
        (&Value::Float32(1.0), &Value::Float64(1.0)),
        (&Value::String("true"), &Value::Boolean(true)),
        (
            &Value::Dynamic(DynamicValue::Integer(DynamicIntegerValue::from_parts(
                false,
                Cow::Borrowed(&[1]),
            ))),
            &Value::Dynamic(DynamicValue::Float32(1.0)),
        ),
        (
            &Value::Dynamic(DynamicValue::Float32(1.0)),
            &Value::Dynamic(DynamicValue::Float64(1.0)),
        ),
        (
            &Value::Dynamic(DynamicValue::Integer(DynamicIntegerValue::from_parts(
                false,
                Cow::Borrowed(&[1]),
            ))),
            &Value::Dynamic(DynamicValue::Float64(1.0)),
        ),
    ];

    for (left, right) in cases {
        assert_eq!(
            equal_scalar_dynamic(left, right),
            Some(Err(ComparisonFailure::DifferentFamily))
        );
        assert_eq!(
            not_equal_scalar_dynamic(left, right),
            Some(Err(ComparisonFailure::DifferentFamily))
        );
        assert_eq!(
            less_scalar_dynamic(left, right),
            Some(Err(ComparisonFailure::DifferentFamily))
        );
        assert_eq!(
            less_equal_scalar_dynamic(left, right),
            Some(Err(ComparisonFailure::DifferentFamily))
        );
        assert_eq!(
            greater_scalar_dynamic(left, right),
            Some(Err(ComparisonFailure::DifferentFamily))
        );
        assert_eq!(
            greater_equal_scalar_dynamic(left, right),
            Some(Err(ComparisonFailure::DifferentFamily))
        );
    }
}

// ============================================================================
// 2. Boolean tests (Section 23)
// ============================================================================

#[test]
fn test_boolean_comparisons() {
    let f = Value::Boolean(false);
    let t = Value::Boolean(true);

    // Equal
    assert_eq!(equal_scalar_dynamic(&f, &f), Some(Ok(true)));
    assert_eq!(equal_scalar_dynamic(&f, &t), Some(Ok(false)));
    assert_eq!(equal_scalar_dynamic(&t, &f), Some(Ok(false)));
    assert_eq!(equal_scalar_dynamic(&t, &t), Some(Ok(true)));

    // NotEqual
    assert_eq!(not_equal_scalar_dynamic(&f, &f), Some(Ok(false)));
    assert_eq!(not_equal_scalar_dynamic(&f, &t), Some(Ok(true)));
    assert_eq!(not_equal_scalar_dynamic(&t, &f), Some(Ok(true)));
    assert_eq!(not_equal_scalar_dynamic(&t, &t), Some(Ok(false)));

    // Less (false < true)
    assert_eq!(less_scalar_dynamic(&f, &f), Some(Ok(false)));
    assert_eq!(less_scalar_dynamic(&f, &t), Some(Ok(true)));
    assert_eq!(less_scalar_dynamic(&t, &f), Some(Ok(false)));
    assert_eq!(less_scalar_dynamic(&t, &t), Some(Ok(false)));

    // LessEqual
    assert_eq!(less_equal_scalar_dynamic(&f, &f), Some(Ok(true)));
    assert_eq!(less_equal_scalar_dynamic(&f, &t), Some(Ok(true)));
    assert_eq!(less_equal_scalar_dynamic(&t, &f), Some(Ok(false)));
    assert_eq!(less_equal_scalar_dynamic(&t, &t), Some(Ok(true)));

    // Greater
    assert_eq!(greater_scalar_dynamic(&f, &f), Some(Ok(false)));
    assert_eq!(greater_scalar_dynamic(&f, &t), Some(Ok(false)));
    assert_eq!(greater_scalar_dynamic(&t, &f), Some(Ok(true)));
    assert_eq!(greater_scalar_dynamic(&t, &t), Some(Ok(false)));

    // GreaterEqual
    assert_eq!(greater_equal_scalar_dynamic(&f, &f), Some(Ok(true)));
    assert_eq!(greater_equal_scalar_dynamic(&f, &t), Some(Ok(false)));
    assert_eq!(greater_equal_scalar_dynamic(&t, &f), Some(Ok(true)));
    assert_eq!(greater_equal_scalar_dynamic(&t, &t), Some(Ok(true)));
}

// ============================================================================
// 3. String tests (Section 24)
// ============================================================================

#[test]
fn test_string_comparisons() {
    let abc = Value::String("abc");
    let abc2 = Value::String("abc");
    let abd = Value::String("abd");
    let capital = Value::String("ABC");

    assert_eq!(equal_scalar_dynamic(&abc, &abc2), Some(Ok(true)));
    assert_eq!(equal_scalar_dynamic(&abc, &abd), Some(Ok(false)));
    assert_eq!(equal_scalar_dynamic(&abc, &capital), Some(Ok(false)));

    assert_eq!(not_equal_scalar_dynamic(&abc, &abc2), Some(Ok(false)));
    assert_eq!(not_equal_scalar_dynamic(&abc, &capital), Some(Ok(true)));

    assert_eq!(less_scalar_dynamic(&abc, &abd), Some(Ok(true)));
    assert_eq!(less_scalar_dynamic(&abd, &abc), Some(Ok(false)));
    assert_eq!(less_scalar_dynamic(&capital, &abc), Some(Ok(true))); // 'A' < 'a'

    assert_eq!(less_equal_scalar_dynamic(&abc, &abc2), Some(Ok(true)));
    assert_eq!(less_equal_scalar_dynamic(&abc, &abd), Some(Ok(true)));
    assert_eq!(less_equal_scalar_dynamic(&abd, &abc), Some(Ok(false)));

    assert_eq!(greater_scalar_dynamic(&abd, &abc), Some(Ok(true)));
    assert_eq!(greater_scalar_dynamic(&abc, &abd), Some(Ok(false)));

    assert_eq!(greater_equal_scalar_dynamic(&abc, &abc2), Some(Ok(true)));
    assert_eq!(greater_equal_scalar_dynamic(&abd, &abc), Some(Ok(true)));
    assert_eq!(greater_equal_scalar_dynamic(&abc, &abd), Some(Ok(false)));

    // Unicode exact & no normalization
    let crab1 = Value::String("🦀");
    let crab2 = Value::String("🦀");
    let fish = Value::String("🐟");
    assert_eq!(equal_scalar_dynamic(&crab1, &crab2), Some(Ok(true)));
    assert_eq!(not_equal_scalar_dynamic(&crab1, &fish), Some(Ok(true)));

    let nfc = Value::String("é"); // U+00E9
    let nfd = Value::String("e\u{0301}"); // 'e' + combining acute
    assert_eq!(equal_scalar_dynamic(&nfc, &nfd), Some(Ok(false)));
    assert_eq!(not_equal_scalar_dynamic(&nfc, &nfd), Some(Ok(true)));
}

// ============================================================================
// 4. Fixed Integer tests (Section 25)
// ============================================================================

macro_rules! test_fixed_integer_comparisons {
    ($test_name:ident, $variant:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            let zero = Value::$variant(0);
            let one = Value::$variant(1);
            let two = Value::$variant(2);
            let min = Value::$variant(<$t>::MIN);
            let max = Value::$variant(<$t>::MAX);

            // Equal & NotEqual
            assert_eq!(equal_scalar_dynamic(&one, &one), Some(Ok(true)));
            assert_eq!(equal_scalar_dynamic(&one, &two), Some(Ok(false)));
            assert_eq!(not_equal_scalar_dynamic(&one, &two), Some(Ok(true)));
            assert_eq!(not_equal_scalar_dynamic(&one, &one), Some(Ok(false)));

            // Less & LessEqual
            assert_eq!(less_scalar_dynamic(&one, &two), Some(Ok(true)));
            assert_eq!(less_scalar_dynamic(&two, &one), Some(Ok(false)));
            assert_eq!(less_scalar_dynamic(&one, &one), Some(Ok(false)));

            assert_eq!(less_equal_scalar_dynamic(&one, &two), Some(Ok(true)));
            assert_eq!(less_equal_scalar_dynamic(&one, &one), Some(Ok(true)));
            assert_eq!(less_equal_scalar_dynamic(&two, &one), Some(Ok(false)));

            // Greater & GreaterEqual
            assert_eq!(greater_scalar_dynamic(&two, &one), Some(Ok(true)));
            assert_eq!(greater_scalar_dynamic(&one, &two), Some(Ok(false)));
            assert_eq!(greater_scalar_dynamic(&one, &one), Some(Ok(false)));

            assert_eq!(greater_equal_scalar_dynamic(&two, &one), Some(Ok(true)));
            assert_eq!(greater_equal_scalar_dynamic(&one, &one), Some(Ok(true)));
            assert_eq!(greater_equal_scalar_dynamic(&one, &two), Some(Ok(false)));

            // Boundary checks
            assert_eq!(
                less_scalar_dynamic(&min, &max),
                Some(Ok(<$t>::MIN < <$t>::MAX))
            );
            assert_eq!(
                greater_scalar_dynamic(&max, &min),
                Some(Ok(<$t>::MAX > <$t>::MIN))
            );
            assert_eq!(equal_scalar_dynamic(&zero, &zero), Some(Ok(true)));
        }
    };
}

test_fixed_integer_comparisons!(test_cmp_i8, Int8, i8);
test_fixed_integer_comparisons!(test_cmp_i16, Int16, i16);
test_fixed_integer_comparisons!(test_cmp_i32, Int32, i32);
test_fixed_integer_comparisons!(test_cmp_i64, Int64, i64);
test_fixed_integer_comparisons!(test_cmp_i128, Int128, i128);

test_fixed_integer_comparisons!(test_cmp_u8, Uint8, u8);
test_fixed_integer_comparisons!(test_cmp_u16, Uint16, u16);
test_fixed_integer_comparisons!(test_cmp_u32, Uint32, u32);
test_fixed_integer_comparisons!(test_cmp_u64, Uint64, u64);
test_fixed_integer_comparisons!(test_cmp_u128, Uint128, u128);

// ============================================================================
// 5. Float tests (Sections 26 & 10 & 11)
// ============================================================================

macro_rules! test_float_comparisons {
    ($test_name:ident, $variant:ident, $t:ty) => {
        #[test]
        fn $test_name() {
            let nan = Value::$variant(<$t>::NAN);
            let pos_zero = Value::$variant(0.0 as $t);
            let neg_zero = Value::$variant(-0.0 as $t);
            let one = Value::$variant(1.0 as $t);
            let two = Value::$variant(2.0 as $t);
            let inf = Value::$variant(<$t>::INFINITY);
            let neg_inf = Value::$variant(<$t>::NEG_INFINITY);

            // Normal values
            assert_eq!(equal_scalar_dynamic(&one, &one), Some(Ok(true)));
            assert_eq!(equal_scalar_dynamic(&one, &two), Some(Ok(false)));
            assert_eq!(not_equal_scalar_dynamic(&one, &two), Some(Ok(true)));
            assert_eq!(less_scalar_dynamic(&one, &two), Some(Ok(true)));
            assert_eq!(less_equal_scalar_dynamic(&one, &two), Some(Ok(true)));
            assert_eq!(greater_scalar_dynamic(&two, &one), Some(Ok(true)));
            assert_eq!(greater_equal_scalar_dynamic(&two, &one), Some(Ok(true)));

            // Signed zeros (+0.0 == -0.0)
            assert_eq!(equal_scalar_dynamic(&pos_zero, &neg_zero), Some(Ok(true)));
            assert_eq!(
                not_equal_scalar_dynamic(&pos_zero, &neg_zero),
                Some(Ok(false))
            );
            assert_eq!(less_scalar_dynamic(&pos_zero, &neg_zero), Some(Ok(false)));
            assert_eq!(
                less_equal_scalar_dynamic(&pos_zero, &neg_zero),
                Some(Ok(true))
            );
            assert_eq!(
                greater_scalar_dynamic(&pos_zero, &neg_zero),
                Some(Ok(false))
            );
            assert_eq!(
                greater_equal_scalar_dynamic(&pos_zero, &neg_zero),
                Some(Ok(true))
            );

            // Infinities
            assert_eq!(less_scalar_dynamic(&neg_inf, &inf), Some(Ok(true)));
            assert_eq!(greater_scalar_dynamic(&inf, &one), Some(Ok(true)));
            assert_eq!(less_scalar_dynamic(&neg_inf, &one), Some(Ok(true)));

            // NaN vs NaN
            assert_eq!(equal_scalar_dynamic(&nan, &nan), Some(Ok(false)));
            assert_eq!(not_equal_scalar_dynamic(&nan, &nan), Some(Ok(true)));
            assert_eq!(less_scalar_dynamic(&nan, &nan), Some(Ok(false)));
            assert_eq!(less_equal_scalar_dynamic(&nan, &nan), Some(Ok(false)));
            assert_eq!(greater_scalar_dynamic(&nan, &nan), Some(Ok(false)));
            assert_eq!(greater_equal_scalar_dynamic(&nan, &nan), Some(Ok(false)));

            // NaN left
            assert_eq!(equal_scalar_dynamic(&nan, &one), Some(Ok(false)));
            assert_eq!(not_equal_scalar_dynamic(&nan, &one), Some(Ok(true)));
            assert_eq!(less_scalar_dynamic(&nan, &one), Some(Ok(false)));
            assert_eq!(less_equal_scalar_dynamic(&nan, &one), Some(Ok(false)));
            assert_eq!(greater_scalar_dynamic(&nan, &one), Some(Ok(false)));
            assert_eq!(greater_equal_scalar_dynamic(&nan, &one), Some(Ok(false)));

            // NaN right
            assert_eq!(equal_scalar_dynamic(&one, &nan), Some(Ok(false)));
            assert_eq!(not_equal_scalar_dynamic(&one, &nan), Some(Ok(true)));
            assert_eq!(less_scalar_dynamic(&one, &nan), Some(Ok(false)));
            assert_eq!(less_equal_scalar_dynamic(&one, &nan), Some(Ok(false)));
            assert_eq!(greater_scalar_dynamic(&one, &nan), Some(Ok(false)));
            assert_eq!(greater_equal_scalar_dynamic(&one, &nan), Some(Ok(false)));
        }
    };
}

test_float_comparisons!(test_cmp_f32, Float32, f32);
test_float_comparisons!(test_cmp_f64, Float64, f64);

// ============================================================================
// 6. Dynamic Integer tests (Section 27)
// ============================================================================

#[test]
fn test_dynamic_integer_comparisons() {
    let make_dyn_int = |neg: bool, mag: &'static [u8]| -> Value<'static> {
        Value::Dynamic(DynamicValue::Integer(DynamicIntegerValue::from_parts(
            neg,
            Cow::Borrowed(mag),
        )))
    };

    let zero = make_dyn_int(false, &[]);
    let zero_neg = make_dyn_int(true, &[]); // canonicalized to non-negative zero
    let one = make_dyn_int(false, &[1]);
    let two = make_dyn_int(false, &[2]);
    let neg_one = make_dyn_int(true, &[1]);
    let neg_two = make_dyn_int(true, &[2]);

    let val_255 = make_dyn_int(false, &[255]);
    let val_256 = make_dyn_int(false, &[1, 0]);
    let neg_255 = make_dyn_int(true, &[255]);
    let neg_256 = make_dyn_int(true, &[1, 0]);

    // 0 == 0
    assert_eq!(equal_scalar_dynamic(&zero, &zero), Some(Ok(true)));
    assert_eq!(equal_scalar_dynamic(&zero, &zero_neg), Some(Ok(true)));

    // 1 == 1, 1 != 2
    assert_eq!(equal_scalar_dynamic(&one, &one), Some(Ok(true)));
    assert_eq!(not_equal_scalar_dynamic(&one, &two), Some(Ok(true)));
    assert_eq!(equal_scalar_dynamic(&one, &two), Some(Ok(false)));

    // negative vs positive
    assert_eq!(less_scalar_dynamic(&neg_one, &one), Some(Ok(true)));
    assert_eq!(greater_scalar_dynamic(&one, &neg_one), Some(Ok(true)));

    // -1 < 0, 0 < 1
    assert_eq!(less_scalar_dynamic(&neg_one, &zero), Some(Ok(true)));
    assert_eq!(less_scalar_dynamic(&zero, &one), Some(Ok(true)));
    assert_eq!(greater_scalar_dynamic(&zero, &neg_one), Some(Ok(true)));
    assert_eq!(greater_scalar_dynamic(&one, &zero), Some(Ok(true)));

    // 2 < 255, 255 < 256
    assert_eq!(less_scalar_dynamic(&two, &val_255), Some(Ok(true)));
    assert_eq!(less_scalar_dynamic(&val_255, &val_256), Some(Ok(true)));
    assert_eq!(greater_scalar_dynamic(&val_256, &val_255), Some(Ok(true)));

    // Magnitude with different byte counts
    let b1 = make_dyn_int(false, &[0x01]);
    let b2 = make_dyn_int(false, &[0x01, 0x00]);
    let b3 = make_dyn_int(false, &[0x01, 0x00, 0x00]);
    assert_eq!(less_scalar_dynamic(&b1, &b2), Some(Ok(true)));
    assert_eq!(less_scalar_dynamic(&b2, &b3), Some(Ok(true)));

    // Same length with lexicographical difference
    let lx1 = make_dyn_int(false, &[0x01, 0x05]);
    let lx2 = make_dyn_int(false, &[0x01, 0x06]);
    assert_eq!(less_scalar_dynamic(&lx1, &lx2), Some(Ok(true)));
    assert_eq!(greater_scalar_dynamic(&lx2, &lx1), Some(Ok(true)));

    // Negative reversal: -256 < -255 and -255 < -2
    assert_eq!(less_scalar_dynamic(&neg_256, &neg_255), Some(Ok(true)));
    assert_eq!(less_scalar_dynamic(&neg_255, &neg_two), Some(Ok(true)));
    assert_eq!(greater_scalar_dynamic(&neg_two, &neg_255), Some(Ok(true)));
    assert_eq!(greater_scalar_dynamic(&neg_255, &neg_256), Some(Ok(true)));

    // Integers greater than u128 (u128 is 16 bytes, here 20 bytes)
    static HUGE_A: [u8; 20] = [1; 20];
    static HUGE_B: [u8; 20] = [2; 20];
    static HUGE_C: [u8; 21] = [1; 21];

    let huge_a = make_dyn_int(false, &HUGE_A);
    let huge_b = make_dyn_int(false, &HUGE_B);
    let huge_c = make_dyn_int(false, &HUGE_C);

    assert_eq!(less_scalar_dynamic(&huge_a, &huge_b), Some(Ok(true)));
    assert_eq!(less_scalar_dynamic(&huge_b, &huge_c), Some(Ok(true)));
    assert_eq!(equal_scalar_dynamic(&huge_a, &huge_a), Some(Ok(true)));

    // Negative integers with magnitude > u128
    let neg_huge_a = make_dyn_int(true, &HUGE_A);
    let neg_huge_b = make_dyn_int(true, &HUGE_B);
    let neg_huge_c = make_dyn_int(true, &HUGE_C);

    assert_eq!(
        less_scalar_dynamic(&neg_huge_b, &neg_huge_a),
        Some(Ok(true))
    );
    assert_eq!(
        less_scalar_dynamic(&neg_huge_c, &neg_huge_b),
        Some(Ok(true))
    );
}

// ============================================================================
// 7. Dynamic Float tests (Section 28)
// ============================================================================

#[test]
fn test_dynamic_float_comparisons() {
    let f32_nan = Value::Dynamic(DynamicValue::Float32(f32::NAN));
    let f32_one = Value::Dynamic(DynamicValue::Float32(1.0));
    let f32_two = Value::Dynamic(DynamicValue::Float32(2.0));

    assert_eq!(equal_scalar_dynamic(&f32_one, &f32_one), Some(Ok(true)));
    assert_eq!(less_scalar_dynamic(&f32_one, &f32_two), Some(Ok(true)));
    assert_eq!(equal_scalar_dynamic(&f32_nan, &f32_nan), Some(Ok(false)));
    assert_eq!(not_equal_scalar_dynamic(&f32_nan, &f32_nan), Some(Ok(true)));
    assert_eq!(less_scalar_dynamic(&f32_nan, &f32_one), Some(Ok(false)));

    let f64_nan = Value::Dynamic(DynamicValue::Float64(f64::NAN));
    let f64_one = Value::Dynamic(DynamicValue::Float64(1.0));
    let f64_two = Value::Dynamic(DynamicValue::Float64(2.0));

    assert_eq!(equal_scalar_dynamic(&f64_one, &f64_one), Some(Ok(true)));
    assert_eq!(less_scalar_dynamic(&f64_one, &f64_two), Some(Ok(true)));
    assert_eq!(equal_scalar_dynamic(&f64_nan, &f64_nan), Some(Ok(false)));
    assert_eq!(not_equal_scalar_dynamic(&f64_nan, &f64_nan), Some(Ok(true)));
    assert_eq!(less_scalar_dynamic(&f64_nan, &f64_one), Some(Ok(false)));
}

// ============================================================================
// 8. Composites deferral and incompatibility tests (Section 29)
// ============================================================================

#[test]
fn test_composite_deferral_and_incompatibility() {
    let st1 = Value::Struct(Box::new([]));
    let st2 = Value::Struct(Box::new([]));

    let en1 = Value::Enum {
        variant: 0,
        payload: EnumPayload::Simple,
    };
    let en2 = Value::Enum {
        variant: 1,
        payload: EnumPayload::Simple,
    };

    let scalar = Value::Boolean(true);

    // Struct vs Struct -> None (deferred to TASK-EV-013)
    assert_eq!(equal_scalar_dynamic(&st1, &st2), None);
    assert_eq!(not_equal_scalar_dynamic(&st1, &st2), None);
    assert_eq!(less_scalar_dynamic(&st1, &st2), None);
    assert_eq!(less_equal_scalar_dynamic(&st1, &st2), None);
    assert_eq!(greater_scalar_dynamic(&st1, &st2), None);
    assert_eq!(greater_equal_scalar_dynamic(&st1, &st2), None);

    // Enum vs Enum -> None (deferred to TASK-EV-013)
    assert_eq!(equal_scalar_dynamic(&en1, &en2), None);
    assert_eq!(not_equal_scalar_dynamic(&en1, &en2), None);
    assert_eq!(less_scalar_dynamic(&en1, &en2), None);
    assert_eq!(less_equal_scalar_dynamic(&en1, &en2), None);
    assert_eq!(greater_scalar_dynamic(&en1, &en2), None);
    assert_eq!(greater_equal_scalar_dynamic(&en1, &en2), None);

    // Struct vs Enum -> DifferentFamily
    assert_eq!(
        equal_scalar_dynamic(&st1, &en1),
        Some(Err(ComparisonFailure::DifferentFamily))
    );
    assert_eq!(
        equal_scalar_dynamic(&en1, &st1),
        Some(Err(ComparisonFailure::DifferentFamily))
    );

    // Struct vs Scalar -> DifferentFamily
    assert_eq!(
        equal_scalar_dynamic(&st1, &scalar),
        Some(Err(ComparisonFailure::DifferentFamily))
    );
    assert_eq!(
        equal_scalar_dynamic(&scalar, &st1),
        Some(Err(ComparisonFailure::DifferentFamily))
    );

    // Enum vs Scalar -> DifferentFamily
    assert_eq!(
        equal_scalar_dynamic(&en1, &scalar),
        Some(Err(ComparisonFailure::DifferentFamily))
    );
    assert_eq!(
        equal_scalar_dynamic(&scalar, &en1),
        Some(Err(ComparisonFailure::DifferentFamily))
    );
}

// ============================================================================
// 9. Function Pointer Contracts
// ============================================================================

#[test]
fn test_function_pointer_types_available() {
    fn check_borrowed_type<F: 'static>(_op: Option<F>) {}
    fn check_owned_type<F: 'static>(_op: Option<F>) {}

    check_borrowed_type::<Equal>(None);
    check_borrowed_type::<NotEqual>(None);
    check_borrowed_type::<Less>(None);
    check_borrowed_type::<LessEqual>(None);
    check_borrowed_type::<Greater>(None);
    check_borrowed_type::<GreaterEqual>(None);
    check_borrowed_type::<ValueComparison>(None);

    check_owned_type::<OwnedEqual>(None);
    check_owned_type::<OwnedNotEqual>(None);
    check_owned_type::<OwnedLess>(None);
    check_owned_type::<OwnedLessEqual>(None);
    check_owned_type::<OwnedGreater>(None);
    check_owned_type::<OwnedGreaterEqual>(None);
    check_owned_type::<OwnedValueComparison>(None);
}
