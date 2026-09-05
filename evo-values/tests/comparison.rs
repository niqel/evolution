extern crate alloc;

use alloc::borrow::Cow;
use alloc::boxed::Box;

use evo_values::comparison::{
    EQUAL, GREATER, GREATER_EQUAL, LESS, LESS_EQUAL, NOT_EQUAL, OWNED_EQUAL, OWNED_GREATER,
    OWNED_GREATER_EQUAL, OWNED_LESS, OWNED_LESS_EQUAL, OWNED_NOT_EQUAL, equal, greater,
    greater_equal, less, less_equal, not_equal, owned_equal, owned_greater, owned_greater_equal,
    owned_less, owned_less_equal, owned_not_equal,
};
use evo_values::definitions::comparison::{
    Equal, Greater, GreaterEqual, Less, LessEqual, NotEqual, OwnedEqual, OwnedGreater,
    OwnedGreaterEqual, OwnedLess, OwnedLessEqual, OwnedNotEqual,
};
use evo_values::definitions::failures::ComparisonFailure;
use evo_values::definitions::value::{
    DynamicIntegerValue, DynamicValue, EnumPayload, OwnedDynamicInteger, OwnedDynamicValue,
    OwnedEnumPayload, OwnedValue, Value,
};

// ============================================================================
// Helper assertions
// ============================================================================

fn assert_cmp_borrowed(
    left: &Value<'_>,
    right: &Value<'_>,
    expected_eq: Result<bool, ComparisonFailure>,
    expected_ne: Result<bool, ComparisonFailure>,
    expected_lt: Result<bool, ComparisonFailure>,
    expected_le: Result<bool, ComparisonFailure>,
    expected_gt: Result<bool, ComparisonFailure>,
    expected_ge: Result<bool, ComparisonFailure>,
) {
    assert_eq!(equal(left, right), expected_eq, "equal mismatch");
    assert_eq!(not_equal(left, right), expected_ne, "not_equal mismatch");
    assert_eq!(less(left, right), expected_lt, "less mismatch");
    assert_eq!(less_equal(left, right), expected_le, "less_equal mismatch");
    assert_eq!(greater(left, right), expected_gt, "greater mismatch");
    assert_eq!(
        greater_equal(left, right),
        expected_ge,
        "greater_equal mismatch"
    );
}

fn assert_cmp_owned(
    left: &OwnedValue,
    right: &OwnedValue,
    expected_eq: Result<bool, ComparisonFailure>,
    expected_ne: Result<bool, ComparisonFailure>,
    expected_lt: Result<bool, ComparisonFailure>,
    expected_le: Result<bool, ComparisonFailure>,
    expected_gt: Result<bool, ComparisonFailure>,
    expected_ge: Result<bool, ComparisonFailure>,
) {
    assert_eq!(
        owned_equal(left, right),
        expected_eq,
        "owned_equal mismatch"
    );
    assert_eq!(
        owned_not_equal(left, right),
        expected_ne,
        "owned_not_equal mismatch"
    );
    assert_eq!(owned_less(left, right), expected_lt, "owned_less mismatch");
    assert_eq!(
        owned_less_equal(left, right),
        expected_le,
        "owned_less_equal mismatch"
    );
    assert_eq!(
        owned_greater(left, right),
        expected_gt,
        "owned_greater mismatch"
    );
    assert_eq!(
        owned_greater_equal(left, right),
        expected_ge,
        "owned_greater_equal mismatch"
    );
}

// ============================================================================
// 1. Borrowed Struct Tests (Section 31)
// ============================================================================

#[test]
fn test_borrowed_struct_empty() {
    let s1 = Value::Struct(Box::new([]));
    let s2 = Value::Struct(Box::new([]));

    assert_cmp_borrowed(
        &s1,
        &s2,
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(true),
    );
}

#[test]
fn test_borrowed_struct_equal_fields() {
    let s1 = Value::Struct(Box::new([Value::Int32(1), Value::String("abc")]));
    let s2 = Value::Struct(Box::new([Value::Int32(1), Value::String("abc")]));

    assert_cmp_borrowed(
        &s1,
        &s2,
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(true),
    );
}

#[test]
fn test_borrowed_struct_different_field() {
    let s1 = Value::Struct(Box::new([Value::Int32(1)]));
    let s2 = Value::Struct(Box::new([Value::Int32(2)]));

    assert_cmp_borrowed(
        &s1,
        &s2,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
}

#[test]
fn test_borrowed_struct_lexicographic_less_and_greater() {
    let s1 = Value::Struct(Box::new([Value::Int32(1), Value::Int32(10)]));
    let s2 = Value::Struct(Box::new([Value::Int32(2), Value::Int32(5)]));

    // s1 < s2 because 1 < 2
    assert_cmp_borrowed(
        &s1,
        &s2,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );

    // s2 > s1 because 2 > 1
    assert_cmp_borrowed(
        &s2,
        &s1,
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(true),
    );
}

#[test]
fn test_borrowed_struct_same_prefix_later_decisive() {
    let s1 = Value::Struct(Box::new([Value::Int32(1), Value::Int32(10)]));
    let s2 = Value::Struct(Box::new([Value::Int32(1), Value::Int32(20)]));

    assert_cmp_borrowed(
        &s1,
        &s2,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
}

#[test]
fn test_borrowed_struct_shape_mismatch() {
    let s1 = Value::Struct(Box::new([Value::Int32(1)]));
    let s2 = Value::Struct(Box::new([Value::Int32(1), Value::Int32(2)]));

    assert_cmp_borrowed(
        &s1,
        &s2,
        Err(ComparisonFailure::NotComparable),
        Err(ComparisonFailure::NotComparable),
        Err(ComparisonFailure::NotComparable),
        Err(ComparisonFailure::NotComparable),
        Err(ComparisonFailure::NotComparable),
        Err(ComparisonFailure::NotComparable),
    );
}

#[test]
fn test_borrowed_struct_nested() {
    let s1 = Value::Struct(Box::new([
        Value::Struct(Box::new([Value::Int32(1), Value::Int32(2)])),
        Value::Int32(3),
    ]));
    let s2 = Value::Struct(Box::new([
        Value::Struct(Box::new([Value::Int32(1), Value::Int32(2)])),
        Value::Int32(4),
    ]));

    assert_cmp_borrowed(
        &s1,
        &s2,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
}

#[test]
fn test_borrowed_struct_nested_enum() {
    let s1 = Value::Struct(Box::new([Value::Enum {
        variant: 0,
        payload: EnumPayload::Simple,
    }]));
    let s2 = Value::Struct(Box::new([Value::Enum {
        variant: 1,
        payload: EnumPayload::Simple,
    }]));

    assert_cmp_borrowed(
        &s1,
        &s2,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
}

#[test]
fn test_borrowed_struct_different_family_field() {
    let s1 = Value::Struct(Box::new([Value::Int32(1), Value::Int32(2)]));
    let s2 = Value::Struct(Box::new([Value::Int32(1), Value::String("two")]));

    assert_cmp_borrowed(
        &s1,
        &s2,
        Err(ComparisonFailure::DifferentFamily),
        Err(ComparisonFailure::DifferentFamily),
        Err(ComparisonFailure::DifferentFamily),
        Err(ComparisonFailure::DifferentFamily),
        Err(ComparisonFailure::DifferentFamily),
        Err(ComparisonFailure::DifferentFamily),
    );
}

#[test]
fn test_borrowed_struct_float_nan() {
    let s1 = Value::Struct(Box::new([Value::Float64(f64::NAN)]));
    let s2 = Value::Struct(Box::new([Value::Float64(f64::NAN)]));

    assert_cmp_borrowed(
        &s1,
        &s2,
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(false),
        Ok(false),
    );

    let s3 = Value::Struct(Box::new([Value::Float64(1.0), Value::Float64(f64::NAN)]));
    let s4 = Value::Struct(Box::new([Value::Float64(1.0), Value::Float64(f64::NAN)]));

    assert_cmp_borrowed(
        &s3,
        &s4,
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(false),
        Ok(false),
    );
}

// ============================================================================
// 2. Borrowed Enum Tests (Section 32)
// ============================================================================

#[test]
fn test_borrowed_enum_simple_same_variant() {
    let e1 = Value::Enum {
        variant: 1,
        payload: EnumPayload::Simple,
    };
    let e2 = Value::Enum {
        variant: 1,
        payload: EnumPayload::Simple,
    };

    assert_cmp_borrowed(
        &e1,
        &e2,
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(true),
    );
}

#[test]
fn test_borrowed_enum_different_variant_ordinal() {
    let e1 = Value::Enum {
        variant: 1,
        payload: EnumPayload::Simple,
    };
    let e2 = Value::Enum {
        variant: 2,
        payload: EnumPayload::Simple,
    };

    assert_cmp_borrowed(
        &e1,
        &e2,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );

    assert_cmp_borrowed(
        &e2,
        &e1,
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(true),
    );
}

#[test]
fn test_borrowed_enum_associated_same_variant() {
    let e1 = Value::Enum {
        variant: 0,
        payload: EnumPayload::Associated(Box::new(Value::Int32(10))),
    };
    let e2 = Value::Enum {
        variant: 0,
        payload: EnumPayload::Associated(Box::new(Value::Int32(20))),
    };
    let e3 = Value::Enum {
        variant: 0,
        payload: EnumPayload::Associated(Box::new(Value::Int32(10))),
    };

    assert_cmp_borrowed(
        &e1,
        &e2,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );

    assert_cmp_borrowed(
        &e1,
        &e3,
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(true),
    );
}

#[test]
fn test_borrowed_enum_structured_same_variant() {
    let e1 = Value::Enum {
        variant: 0,
        payload: EnumPayload::Structured {
            fields: Box::new([Value::Int32(1), Value::String("a")]),
        },
    };
    let e2 = Value::Enum {
        variant: 0,
        payload: EnumPayload::Structured {
            fields: Box::new([Value::Int32(1), Value::String("a")]),
        },
    };

    assert_cmp_borrowed(
        &e1,
        &e2,
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(true),
    );
}

#[test]
fn test_borrowed_enum_structured_lexicographic() {
    let e1 = Value::Enum {
        variant: 0,
        payload: EnumPayload::Structured {
            fields: Box::new([Value::Int32(1), Value::Int32(10)]),
        },
    };
    let e2 = Value::Enum {
        variant: 0,
        payload: EnumPayload::Structured {
            fields: Box::new([Value::Int32(1), Value::Int32(20)]),
        },
    };

    assert_cmp_borrowed(
        &e1,
        &e2,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
}

#[test]
fn test_borrowed_enum_payload_kind_mismatch() {
    let simple = Value::Enum {
        variant: 0,
        payload: EnumPayload::Simple,
    };
    let associated = Value::Enum {
        variant: 0,
        payload: EnumPayload::Associated(Box::new(Value::Int32(1))),
    };
    let structured = Value::Enum {
        variant: 0,
        payload: EnumPayload::Structured {
            fields: Box::new([Value::Int32(1)]),
        },
    };

    let pairs = [
        (&simple, &associated),
        (&simple, &structured),
        (&associated, &structured),
    ];

    for (a, b) in pairs {
        assert_cmp_borrowed(
            a,
            b,
            Err(ComparisonFailure::NotComparable),
            Err(ComparisonFailure::NotComparable),
            Err(ComparisonFailure::NotComparable),
            Err(ComparisonFailure::NotComparable),
            Err(ComparisonFailure::NotComparable),
            Err(ComparisonFailure::NotComparable),
        );
    }
}

#[test]
fn test_borrowed_enum_structured_length_mismatch() {
    let e1 = Value::Enum {
        variant: 0,
        payload: EnumPayload::Structured {
            fields: Box::new([Value::Int32(1)]),
        },
    };
    let e2 = Value::Enum {
        variant: 0,
        payload: EnumPayload::Structured {
            fields: Box::new([Value::Int32(1), Value::Int32(2)]),
        },
    };

    assert_cmp_borrowed(
        &e1,
        &e2,
        Err(ComparisonFailure::NotComparable),
        Err(ComparisonFailure::NotComparable),
        Err(ComparisonFailure::NotComparable),
        Err(ComparisonFailure::NotComparable),
        Err(ComparisonFailure::NotComparable),
        Err(ComparisonFailure::NotComparable),
    );
}

#[test]
fn test_borrowed_enum_different_variant_ignores_payload_shape() {
    let e1 = Value::Enum {
        variant: 1,
        payload: EnumPayload::Simple,
    };
    let e2 = Value::Enum {
        variant: 2,
        payload: EnumPayload::Structured {
            fields: Box::new([Value::Int32(1), Value::Int32(2)]),
        },
    };

    assert_cmp_borrowed(
        &e1,
        &e2,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
}

#[test]
fn test_borrowed_enum_nested_struct_payload() {
    let e1 = Value::Enum {
        variant: 0,
        payload: EnumPayload::Associated(Box::new(Value::Struct(Box::new([Value::Int32(1)])))),
    };
    let e2 = Value::Enum {
        variant: 0,
        payload: EnumPayload::Associated(Box::new(Value::Struct(Box::new([Value::Int32(2)])))),
    };

    assert_cmp_borrowed(
        &e1,
        &e2,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
}

#[test]
fn test_borrowed_enum_nested_enum_payload() {
    let e1 = Value::Enum {
        variant: 0,
        payload: EnumPayload::Associated(Box::new(Value::Enum {
            variant: 0,
            payload: EnumPayload::Simple,
        })),
    };
    let e2 = Value::Enum {
        variant: 0,
        payload: EnumPayload::Associated(Box::new(Value::Enum {
            variant: 1,
            payload: EnumPayload::Simple,
        })),
    };

    assert_cmp_borrowed(
        &e1,
        &e2,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
}

#[test]
fn test_borrowed_enum_different_family_in_payload() {
    let e1 = Value::Enum {
        variant: 0,
        payload: EnumPayload::Associated(Box::new(Value::Int32(1))),
    };
    let e2 = Value::Enum {
        variant: 0,
        payload: EnumPayload::Associated(Box::new(Value::String("1"))),
    };

    assert_cmp_borrowed(
        &e1,
        &e2,
        Err(ComparisonFailure::DifferentFamily),
        Err(ComparisonFailure::DifferentFamily),
        Err(ComparisonFailure::DifferentFamily),
        Err(ComparisonFailure::DifferentFamily),
        Err(ComparisonFailure::DifferentFamily),
        Err(ComparisonFailure::DifferentFamily),
    );
}

#[test]
fn test_borrowed_enum_nan_in_payload() {
    let e1 = Value::Enum {
        variant: 0,
        payload: EnumPayload::Associated(Box::new(Value::Float32(f32::NAN))),
    };
    let e2 = Value::Enum {
        variant: 0,
        payload: EnumPayload::Associated(Box::new(Value::Float32(f32::NAN))),
    };

    assert_cmp_borrowed(
        &e1,
        &e2,
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(false),
        Ok(false),
    );
}

// ============================================================================
// 3. Owned Scalar / Dynamic Tests (Section 33)
// ============================================================================

#[test]
fn test_owned_boolean() {
    let f = OwnedValue::Boolean(false);
    let t = OwnedValue::Boolean(true);

    assert_cmp_owned(
        &f,
        &f,
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(true),
    );
    assert_cmp_owned(
        &f,
        &t,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
    assert_cmp_owned(
        &t,
        &f,
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(true),
    );
    assert_cmp_owned(
        &t,
        &t,
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(true),
    );
}

#[test]
fn test_owned_string() {
    let a = OwnedValue::String(Box::from("abc"));
    let b = OwnedValue::String(Box::from("def"));
    let a2 = OwnedValue::String(Box::from("abc"));

    assert_cmp_owned(
        &a,
        &b,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
    assert_cmp_owned(
        &a,
        &a2,
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(true),
    );
}

#[test]
fn test_owned_fixed_integers_all_ten_families() {
    // Signed
    assert_cmp_owned(
        &OwnedValue::Int8(-5),
        &OwnedValue::Int8(5),
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
    assert_cmp_owned(
        &OwnedValue::Int8(5),
        &OwnedValue::Int8(5),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(true),
    );

    assert_cmp_owned(
        &OwnedValue::Int16(-50),
        &OwnedValue::Int16(50),
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
    assert_cmp_owned(
        &OwnedValue::Int16(50),
        &OwnedValue::Int16(50),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(true),
    );

    assert_cmp_owned(
        &OwnedValue::Int32(-500),
        &OwnedValue::Int32(500),
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
    assert_cmp_owned(
        &OwnedValue::Int32(500),
        &OwnedValue::Int32(500),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(true),
    );

    assert_cmp_owned(
        &OwnedValue::Int64(-5000),
        &OwnedValue::Int64(5000),
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
    assert_cmp_owned(
        &OwnedValue::Int64(5000),
        &OwnedValue::Int64(5000),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(true),
    );

    assert_cmp_owned(
        &OwnedValue::Int128(-50000),
        &OwnedValue::Int128(50000),
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
    assert_cmp_owned(
        &OwnedValue::Int128(50000),
        &OwnedValue::Int128(50000),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(true),
    );

    // Unsigned
    assert_cmp_owned(
        &OwnedValue::Uint8(5),
        &OwnedValue::Uint8(10),
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
    assert_cmp_owned(
        &OwnedValue::Uint8(10),
        &OwnedValue::Uint8(10),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(true),
    );

    assert_cmp_owned(
        &OwnedValue::Uint16(50),
        &OwnedValue::Uint16(100),
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
    assert_cmp_owned(
        &OwnedValue::Uint16(100),
        &OwnedValue::Uint16(100),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(true),
    );

    assert_cmp_owned(
        &OwnedValue::Uint32(500),
        &OwnedValue::Uint32(1000),
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
    assert_cmp_owned(
        &OwnedValue::Uint32(1000),
        &OwnedValue::Uint32(1000),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(true),
    );

    assert_cmp_owned(
        &OwnedValue::Uint64(5000),
        &OwnedValue::Uint64(10000),
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
    assert_cmp_owned(
        &OwnedValue::Uint64(10000),
        &OwnedValue::Uint64(10000),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(true),
    );

    assert_cmp_owned(
        &OwnedValue::Uint128(50000),
        &OwnedValue::Uint128(100000),
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
    assert_cmp_owned(
        &OwnedValue::Uint128(100000),
        &OwnedValue::Uint128(100000),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(true),
    );
}

#[test]
fn test_owned_floats_and_nan() {
    let f1 = OwnedValue::Float32(1.0);
    let f2 = OwnedValue::Float32(2.0);
    let fnan = OwnedValue::Float32(f32::NAN);

    assert_cmp_owned(
        &f1,
        &f2,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
    assert_cmp_owned(
        &fnan,
        &fnan,
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(false),
        Ok(false),
    );
    assert_cmp_owned(
        &fnan,
        &f1,
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(false),
        Ok(false),
    );

    let d1 = OwnedValue::Float64(10.0);
    let d2 = OwnedValue::Float64(20.0);
    let dnan = OwnedValue::Float64(f64::NAN);

    assert_cmp_owned(
        &d1,
        &d2,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
    assert_cmp_owned(
        &dnan,
        &dnan,
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(false),
        Ok(false),
    );
}

#[test]
fn test_owned_dynamic_integer() {
    let pos_small = OwnedValue::Dynamic(OwnedDynamicValue::Integer(
        OwnedDynamicInteger::from_parts(false, Box::new([1])),
    ));
    let pos_large = OwnedValue::Dynamic(OwnedDynamicValue::Integer(
        OwnedDynamicInteger::from_parts(false, Box::new([2])),
    ));
    let neg_small = OwnedValue::Dynamic(OwnedDynamicValue::Integer(
        OwnedDynamicInteger::from_parts(true, Box::new([1])),
    ));
    let neg_large = OwnedValue::Dynamic(OwnedDynamicValue::Integer(
        OwnedDynamicInteger::from_parts(true, Box::new([2])),
    ));

    // -2 < -1 < 1 < 2
    assert_cmp_owned(
        &neg_large,
        &neg_small,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
    assert_cmp_owned(
        &neg_small,
        &pos_small,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
    assert_cmp_owned(
        &pos_small,
        &pos_large,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );

    // > u128 magnitude (20 bytes)
    let mag_a = Box::new([0x01; 20]);
    let mut mag_b = Box::new([0x01; 20]);
    mag_b[19] = 0x02;

    let big_a = OwnedValue::Dynamic(OwnedDynamicValue::Integer(OwnedDynamicInteger::from_parts(
        false, mag_a,
    )));
    let big_b = OwnedValue::Dynamic(OwnedDynamicValue::Integer(OwnedDynamicInteger::from_parts(
        false, mag_b,
    )));

    assert_cmp_owned(
        &big_a,
        &big_b,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
    assert_cmp_owned(
        &big_b,
        &big_a,
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(true),
    );
}

#[test]
fn test_owned_dynamic_floats() {
    let f1 = OwnedValue::Dynamic(OwnedDynamicValue::Float32(1.5));
    let f2 = OwnedValue::Dynamic(OwnedDynamicValue::Float32(2.5));
    let fnan = OwnedValue::Dynamic(OwnedDynamicValue::Float32(f32::NAN));

    assert_cmp_owned(
        &f1,
        &f2,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
    assert_cmp_owned(
        &fnan,
        &fnan,
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(false),
        Ok(false),
    );

    let d1 = OwnedValue::Dynamic(OwnedDynamicValue::Float64(1.5));
    let d2 = OwnedValue::Dynamic(OwnedDynamicValue::Float64(2.5));
    let dnan = OwnedValue::Dynamic(OwnedDynamicValue::Float64(f64::NAN));

    assert_cmp_owned(
        &d1,
        &d2,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );
    assert_cmp_owned(
        &dnan,
        &dnan,
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(false),
        Ok(false),
    );
}

#[test]
fn test_owned_different_family() {
    let b = OwnedValue::Boolean(true);
    let i = OwnedValue::Int32(1);
    let s = OwnedValue::String(Box::from("true"));
    let st = OwnedValue::Struct(Box::new([]));
    let en = OwnedValue::Enum {
        variant: 0,
        payload: OwnedEnumPayload::Simple,
    };

    let pairs = [(&b, &i), (&i, &s), (&s, &st), (&st, &en), (&b, &en)];

    for (left, right) in pairs {
        assert_cmp_owned(
            left,
            right,
            Err(ComparisonFailure::DifferentFamily),
            Err(ComparisonFailure::DifferentFamily),
            Err(ComparisonFailure::DifferentFamily),
            Err(ComparisonFailure::DifferentFamily),
            Err(ComparisonFailure::DifferentFamily),
            Err(ComparisonFailure::DifferentFamily),
        );
    }
}

// ============================================================================
// 4. Owned Struct / Enum Tests (Section 34)
// ============================================================================

#[test]
fn test_owned_struct_cases() {
    // Equality
    let s1 = OwnedValue::Struct(Box::new([
        OwnedValue::Int32(1),
        OwnedValue::String(Box::from("x")),
    ]));
    let s2 = OwnedValue::Struct(Box::new([
        OwnedValue::Int32(1),
        OwnedValue::String(Box::from("x")),
    ]));
    assert_cmp_owned(
        &s1,
        &s2,
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(true),
    );

    // Lexicographic
    let s3 = OwnedValue::Struct(Box::new([
        OwnedValue::Int32(1),
        OwnedValue::String(Box::from("y")),
    ]));
    assert_cmp_owned(
        &s1,
        &s3,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );

    // Shape mismatch
    let s_short = OwnedValue::Struct(Box::new([OwnedValue::Int32(1)]));
    assert_cmp_owned(
        &s1,
        &s_short,
        Err(ComparisonFailure::NotComparable),
        Err(ComparisonFailure::NotComparable),
        Err(ComparisonFailure::NotComparable),
        Err(ComparisonFailure::NotComparable),
        Err(ComparisonFailure::NotComparable),
        Err(ComparisonFailure::NotComparable),
    );

    // NaN inside struct
    let snan1 = OwnedValue::Struct(Box::new([OwnedValue::Float64(f64::NAN)]));
    let snan2 = OwnedValue::Struct(Box::new([OwnedValue::Float64(f64::NAN)]));
    assert_cmp_owned(
        &snan1,
        &snan2,
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(false),
        Ok(false),
    );
}

#[test]
fn test_owned_enum_cases() {
    // Ordinal decides
    let e1 = OwnedValue::Enum {
        variant: 1,
        payload: OwnedEnumPayload::Simple,
    };
    let e2 = OwnedValue::Enum {
        variant: 2,
        payload: OwnedEnumPayload::Simple,
    };
    assert_cmp_owned(
        &e1,
        &e2,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );

    // Simple
    let e1_copy = OwnedValue::Enum {
        variant: 1,
        payload: OwnedEnumPayload::Simple,
    };
    assert_cmp_owned(
        &e1,
        &e1_copy,
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(true),
    );

    // Associated
    let ea1 = OwnedValue::Enum {
        variant: 0,
        payload: OwnedEnumPayload::Associated(Box::new(OwnedValue::Int32(10))),
    };
    let ea2 = OwnedValue::Enum {
        variant: 0,
        payload: OwnedEnumPayload::Associated(Box::new(OwnedValue::Int32(20))),
    };
    assert_cmp_owned(
        &ea1,
        &ea2,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );

    // Structured
    let es1 = OwnedValue::Enum {
        variant: 0,
        payload: OwnedEnumPayload::Structured {
            fields: Box::new([OwnedValue::Int32(1), OwnedValue::Int32(2)]),
        },
    };
    let es2 = OwnedValue::Enum {
        variant: 0,
        payload: OwnedEnumPayload::Structured {
            fields: Box::new([OwnedValue::Int32(1), OwnedValue::Int32(3)]),
        },
    };
    assert_cmp_owned(
        &es1,
        &es2,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );

    // Payload mismatch
    let esimple = OwnedValue::Enum {
        variant: 0,
        payload: OwnedEnumPayload::Simple,
    };
    assert_cmp_owned(
        &esimple,
        &ea1,
        Err(ComparisonFailure::NotComparable),
        Err(ComparisonFailure::NotComparable),
        Err(ComparisonFailure::NotComparable),
        Err(ComparisonFailure::NotComparable),
        Err(ComparisonFailure::NotComparable),
        Err(ComparisonFailure::NotComparable),
    );

    // Nested composites
    let enest1 = OwnedValue::Enum {
        variant: 0,
        payload: OwnedEnumPayload::Associated(Box::new(OwnedValue::Struct(Box::new([
            OwnedValue::Int32(1),
        ])))),
    };
    let enest2 = OwnedValue::Enum {
        variant: 0,
        payload: OwnedEnumPayload::Associated(Box::new(OwnedValue::Struct(Box::new([
            OwnedValue::Int32(2),
        ])))),
    };
    assert_cmp_owned(
        &enest1,
        &enest2,
        Ok(false),
        Ok(true),
        Ok(true),
        Ok(true),
        Ok(false),
        Ok(false),
    );

    // NaN inside enum
    let enan1 = OwnedValue::Enum {
        variant: 0,
        payload: OwnedEnumPayload::Associated(Box::new(OwnedValue::Float32(f32::NAN))),
    };
    let enan2 = OwnedValue::Enum {
        variant: 0,
        payload: OwnedEnumPayload::Associated(Box::new(OwnedValue::Float32(f32::NAN))),
    };
    assert_cmp_owned(
        &enan1,
        &enan2,
        Ok(false),
        Ok(true),
        Ok(false),
        Ok(false),
        Ok(false),
        Ok(false),
    );
}

// ============================================================================
// 5. Borrowed / Owned Parity (Section 35)
// ============================================================================

#[test]
fn test_borrowed_owned_parity() {
    // 1. Scalar parity (Int32)
    let b_i1 = Value::Int32(10);
    let b_i2 = Value::Int32(20);
    let o_i1 = OwnedValue::Int32(10);
    let o_i2 = OwnedValue::Int32(20);
    assert_eq!(equal(&b_i1, &b_i2), owned_equal(&o_i1, &o_i2));
    assert_eq!(not_equal(&b_i1, &b_i2), owned_not_equal(&o_i1, &o_i2));
    assert_eq!(less(&b_i1, &b_i2), owned_less(&o_i1, &o_i2));
    assert_eq!(less_equal(&b_i1, &b_i2), owned_less_equal(&o_i1, &o_i2));
    assert_eq!(greater(&b_i1, &b_i2), owned_greater(&o_i1, &o_i2));
    assert_eq!(
        greater_equal(&b_i1, &b_i2),
        owned_greater_equal(&o_i1, &o_i2)
    );

    // 2. Dynamic Integer > u128 parity
    let bytes_a: [u8; 20] = [0x05; 20];
    let bytes_b: [u8; 20] = [0x06; 20];

    let b_dyn1 = Value::Dynamic(DynamicValue::Integer(DynamicIntegerValue::from_parts(
        false,
        Cow::Borrowed(&bytes_a),
    )));
    let b_dyn2 = Value::Dynamic(DynamicValue::Integer(DynamicIntegerValue::from_parts(
        false,
        Cow::Borrowed(&bytes_b),
    )));
    let o_dyn1 = OwnedValue::Dynamic(OwnedDynamicValue::Integer(OwnedDynamicInteger::from_parts(
        false,
        Box::new(bytes_a),
    )));
    let o_dyn2 = OwnedValue::Dynamic(OwnedDynamicValue::Integer(OwnedDynamicInteger::from_parts(
        false,
        Box::new(bytes_b),
    )));

    assert_eq!(equal(&b_dyn1, &b_dyn2), owned_equal(&o_dyn1, &o_dyn2));
    assert_eq!(
        not_equal(&b_dyn1, &b_dyn2),
        owned_not_equal(&o_dyn1, &o_dyn2)
    );
    assert_eq!(less(&b_dyn1, &b_dyn2), owned_less(&o_dyn1, &o_dyn2));
    assert_eq!(
        less_equal(&b_dyn1, &b_dyn2),
        owned_less_equal(&o_dyn1, &o_dyn2)
    );
    assert_eq!(greater(&b_dyn1, &b_dyn2), owned_greater(&o_dyn1, &o_dyn2));
    assert_eq!(
        greater_equal(&b_dyn1, &b_dyn2),
        owned_greater_equal(&o_dyn1, &o_dyn2)
    );

    // 3. Float NaN parity
    let b_nan = Value::Float64(f64::NAN);
    let o_nan = OwnedValue::Float64(f64::NAN);
    assert_eq!(equal(&b_nan, &b_nan), owned_equal(&o_nan, &o_nan));
    assert_eq!(not_equal(&b_nan, &b_nan), owned_not_equal(&o_nan, &o_nan));
    assert_eq!(less(&b_nan, &b_nan), owned_less(&o_nan, &o_nan));
    assert_eq!(less_equal(&b_nan, &b_nan), owned_less_equal(&o_nan, &o_nan));
    assert_eq!(greater(&b_nan, &b_nan), owned_greater(&o_nan, &o_nan));
    assert_eq!(
        greater_equal(&b_nan, &b_nan),
        owned_greater_equal(&o_nan, &o_nan)
    );

    // 4. Struct parity
    let b_st1 = Value::Struct(Box::new([Value::Int32(1), Value::String("foo")]));
    let b_st2 = Value::Struct(Box::new([Value::Int32(1), Value::String("bar")]));
    let o_st1 = OwnedValue::Struct(Box::new([
        OwnedValue::Int32(1),
        OwnedValue::String(Box::from("foo")),
    ]));
    let o_st2 = OwnedValue::Struct(Box::new([
        OwnedValue::Int32(1),
        OwnedValue::String(Box::from("bar")),
    ]));

    assert_eq!(equal(&b_st1, &b_st2), owned_equal(&o_st1, &o_st2));
    assert_eq!(not_equal(&b_st1, &b_st2), owned_not_equal(&o_st1, &o_st2));
    assert_eq!(less(&b_st1, &b_st2), owned_less(&o_st1, &o_st2));
    assert_eq!(less_equal(&b_st1, &b_st2), owned_less_equal(&o_st1, &o_st2));
    assert_eq!(greater(&b_st1, &b_st2), owned_greater(&o_st1, &o_st2));
    assert_eq!(
        greater_equal(&b_st1, &b_st2),
        owned_greater_equal(&o_st1, &o_st2)
    );

    // 5. Enum parity
    let b_en1 = Value::Enum {
        variant: 3,
        payload: EnumPayload::Structured {
            fields: Box::new([Value::Int32(10), Value::Int32(20)]),
        },
    };
    let b_en2 = Value::Enum {
        variant: 3,
        payload: EnumPayload::Structured {
            fields: Box::new([Value::Int32(10), Value::Int32(25)]),
        },
    };
    let o_en1 = OwnedValue::Enum {
        variant: 3,
        payload: OwnedEnumPayload::Structured {
            fields: Box::new([OwnedValue::Int32(10), OwnedValue::Int32(20)]),
        },
    };
    let o_en2 = OwnedValue::Enum {
        variant: 3,
        payload: OwnedEnumPayload::Structured {
            fields: Box::new([OwnedValue::Int32(10), OwnedValue::Int32(25)]),
        },
    };

    assert_eq!(equal(&b_en1, &b_en2), owned_equal(&o_en1, &o_en2));
    assert_eq!(not_equal(&b_en1, &b_en2), owned_not_equal(&o_en1, &o_en2));
    assert_eq!(less(&b_en1, &b_en2), owned_less(&o_en1, &o_en2));
    assert_eq!(less_equal(&b_en1, &b_en2), owned_less_equal(&o_en1, &o_en2));
    assert_eq!(greater(&b_en1, &b_en2), owned_greater(&o_en1, &o_en2));
    assert_eq!(
        greater_equal(&b_en1, &b_en2),
        owned_greater_equal(&o_en1, &o_en2)
    );

    // 6. Failure parity (Shape mismatch)
    let b_shape1 = Value::Struct(Box::new([Value::Int32(1)]));
    let b_shape2 = Value::Struct(Box::new([Value::Int32(1), Value::Int32(2)]));
    let o_shape1 = OwnedValue::Struct(Box::new([OwnedValue::Int32(1)]));
    let o_shape2 = OwnedValue::Struct(Box::new([OwnedValue::Int32(1), OwnedValue::Int32(2)]));

    assert_eq!(
        equal(&b_shape1, &b_shape2),
        owned_equal(&o_shape1, &o_shape2)
    );
    assert_eq!(
        not_equal(&b_shape1, &b_shape2),
        owned_not_equal(&o_shape1, &o_shape2)
    );
    assert_eq!(less(&b_shape1, &b_shape2), owned_less(&o_shape1, &o_shape2));
    assert_eq!(
        less_equal(&b_shape1, &b_shape2),
        owned_less_equal(&o_shape1, &o_shape2)
    );
    assert_eq!(
        greater(&b_shape1, &b_shape2),
        owned_greater(&o_shape1, &o_shape2)
    );
    assert_eq!(
        greater_equal(&b_shape1, &b_shape2),
        owned_greater_equal(&o_shape1, &o_shape2)
    );

    // 7. Failure parity (Different family)
    let b_fam1 = Value::Int32(1);
    let b_fam2 = Value::String("1");
    let o_fam1 = OwnedValue::Int32(1);
    let o_fam2 = OwnedValue::String(Box::from("1"));

    assert_eq!(equal(&b_fam1, &b_fam2), owned_equal(&o_fam1, &o_fam2));
    assert_eq!(
        not_equal(&b_fam1, &b_fam2),
        owned_not_equal(&o_fam1, &o_fam2)
    );
    assert_eq!(less(&b_fam1, &b_fam2), owned_less(&o_fam1, &o_fam2));
    assert_eq!(
        less_equal(&b_fam1, &b_fam2),
        owned_less_equal(&o_fam1, &o_fam2)
    );
    assert_eq!(greater(&b_fam1, &b_fam2), owned_greater(&o_fam1, &o_fam2));
    assert_eq!(
        greater_equal(&b_fam1, &b_fam2),
        owned_greater_equal(&o_fam1, &o_fam2)
    );
}

// ============================================================================
// 6. Public Function Pointer Contracts (Section 36)
// ============================================================================

#[test]
fn test_public_function_pointer_contracts() {
    let eq: Equal = EQUAL;
    let ne: NotEqual = NOT_EQUAL;
    let lt: Less = LESS;
    let le: LessEqual = LESS_EQUAL;
    let gt: Greater = GREATER;
    let ge: GreaterEqual = GREATER_EQUAL;

    let v1 = Value::Int32(10);
    let v2 = Value::Int32(20);

    assert_eq!(eq(&v1, &v2), Ok(false));
    assert_eq!(ne(&v1, &v2), Ok(true));
    assert_eq!(lt(&v1, &v2), Ok(true));
    assert_eq!(le(&v1, &v2), Ok(true));
    assert_eq!(gt(&v1, &v2), Ok(false));
    assert_eq!(ge(&v1, &v2), Ok(false));

    let fn_eq: Equal = equal;
    let fn_ne: NotEqual = not_equal;
    let fn_lt: Less = less;
    let fn_le: LessEqual = less_equal;
    let fn_gt: Greater = greater;
    let fn_ge: GreaterEqual = greater_equal;

    assert_eq!(fn_eq(&v1, &v2), Ok(false));
    assert_eq!(fn_ne(&v1, &v2), Ok(true));
    assert_eq!(fn_lt(&v1, &v2), Ok(true));
    assert_eq!(fn_le(&v1, &v2), Ok(true));
    assert_eq!(fn_gt(&v1, &v2), Ok(false));
    assert_eq!(fn_ge(&v1, &v2), Ok(false));

    let owned_eq: OwnedEqual = OWNED_EQUAL;
    let owned_ne: OwnedNotEqual = OWNED_NOT_EQUAL;
    let owned_lt: OwnedLess = OWNED_LESS;
    let owned_le: OwnedLessEqual = OWNED_LESS_EQUAL;
    let owned_gt: OwnedGreater = OWNED_GREATER;
    let owned_ge: OwnedGreaterEqual = OWNED_GREATER_EQUAL;

    let ov1 = OwnedValue::Int32(10);
    let ov2 = OwnedValue::Int32(20);

    assert_eq!(owned_eq(&ov1, &ov2), Ok(false));
    assert_eq!(owned_ne(&ov1, &ov2), Ok(true));
    assert_eq!(owned_lt(&ov1, &ov2), Ok(true));
    assert_eq!(owned_le(&ov1, &ov2), Ok(true));
    assert_eq!(owned_gt(&ov1, &ov2), Ok(false));
    assert_eq!(owned_ge(&ov1, &ov2), Ok(false));

    let fn_owned_eq: OwnedEqual = owned_equal;
    let fn_owned_ne: OwnedNotEqual = owned_not_equal;
    let fn_owned_lt: OwnedLess = owned_less;
    let fn_owned_le: OwnedLessEqual = owned_less_equal;
    let fn_owned_gt: OwnedGreater = owned_greater;
    let fn_owned_ge: OwnedGreaterEqual = owned_greater_equal;

    assert_eq!(fn_owned_eq(&ov1, &ov2), Ok(false));
    assert_eq!(fn_owned_ne(&ov1, &ov2), Ok(true));
    assert_eq!(fn_owned_lt(&ov1, &ov2), Ok(true));
    assert_eq!(fn_owned_le(&ov1, &ov2), Ok(true));
    assert_eq!(fn_owned_gt(&ov1, &ov2), Ok(false));
    assert_eq!(fn_owned_ge(&ov1, &ov2), Ok(false));
}
