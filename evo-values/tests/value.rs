extern crate alloc;

use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::string::String as AllocString;
use alloc::vec;
use evo_values::{
    DynamicIntegerValue, DynamicValue, EnumPayload, OwnedDynamicInteger, OwnedDynamicValue,
    OwnedEnumPayload, OwnedValue, Value,
};

// ============================================================================
// 1. Exact 17 Borrowed Value Families
// ============================================================================

#[test]
fn borrowed_17_families_explicit_coverage() {
    let families: [Value; 17] = [
        Value::Boolean(true),
        Value::Int8(1),
        Value::Int16(2),
        Value::Int32(3),
        Value::Int64(4),
        Value::Int128(5),
        Value::Uint8(6),
        Value::Uint16(7),
        Value::Uint32(8),
        Value::Uint64(9),
        Value::Uint128(10),
        Value::Float32(11.0),
        Value::Float64(12.0),
        Value::String("thirteen"),
        Value::Dynamic(DynamicValue::Float64(14.0)),
        Value::Struct(Box::new([Value::Int32(15)])),
        Value::Enum {
            variant: 16,
            payload: EnumPayload::Simple,
        },
    ];

    assert_eq!(families.len(), 17);
    assert_eq!(families[0], Value::Boolean(true));
    assert_eq!(families[13], Value::String("thirteen"));
    assert_eq!(families[15], Value::Struct(Box::new([Value::Int32(15)])));
}

#[test]
fn borrowed_boolean_variants() {
    assert_eq!(Value::Boolean(true), Value::Boolean(true));
    assert_ne!(Value::Boolean(true), Value::Boolean(false));
}

#[test]
fn borrowed_signed_integer_variants() {
    assert_eq!(Value::Int8(42), Value::Int8(42));
    assert_ne!(Value::Int8(42), Value::Int8(-42));

    assert_eq!(Value::Int16(1000), Value::Int16(1000));
    assert_ne!(Value::Int16(1000), Value::Int16(-1000));

    assert_eq!(Value::Int32(100_000), Value::Int32(100_000));
    assert_ne!(Value::Int32(100_000), Value::Int32(-100_000));

    assert_eq!(Value::Int64(10_000_000_000), Value::Int64(10_000_000_000));
    assert_ne!(Value::Int64(10_000_000_000), Value::Int64(-10_000_000_000));

    assert_eq!(
        Value::Int128(1_000_000_000_000_000_000_000),
        Value::Int128(1_000_000_000_000_000_000_000)
    );
    assert_ne!(
        Value::Int128(1_000_000_000_000_000_000_000),
        Value::Int128(-1_000_000_000_000_000_000_000)
    );
}

#[test]
fn borrowed_unsigned_integer_variants() {
    assert_eq!(Value::Uint8(255), Value::Uint8(255));
    assert_ne!(Value::Uint8(255), Value::Uint8(0));

    assert_eq!(Value::Uint16(65535), Value::Uint16(65535));
    assert_ne!(Value::Uint16(65535), Value::Uint16(0));

    assert_eq!(Value::Uint32(4_000_000_000), Value::Uint32(4_000_000_000));
    assert_ne!(Value::Uint32(4_000_000_000), Value::Uint32(0));

    assert_eq!(
        Value::Uint64(18_000_000_000_000_000_000),
        Value::Uint64(18_000_000_000_000_000_000)
    );
    assert_ne!(Value::Uint64(18_000_000_000_000_000_000), Value::Uint64(0));

    assert_eq!(
        Value::Uint128(300_000_000_000_000_000_000_000_000_000_000_000),
        Value::Uint128(300_000_000_000_000_000_000_000_000_000_000_000)
    );
    assert_ne!(
        Value::Uint128(300_000_000_000_000_000_000_000_000_000_000_000),
        Value::Uint128(0)
    );
}

#[test]
fn borrowed_float_variants() {
    assert_eq!(Value::Float32(1.25), Value::Float32(1.25));
    assert_ne!(Value::Float32(1.25), Value::Float32(2.5));

    assert_eq!(
        Value::Float64(123.456789012345),
        Value::Float64(123.456789012345)
    );
    assert_ne!(
        Value::Float64(123.456789012345),
        Value::Float64(987.654321098765)
    );
}

#[test]
fn borrowed_string_variant() {
    let text = "hola mundo";
    assert_eq!(Value::String(text), Value::String("hola mundo"));
    assert_ne!(Value::String(text), Value::String("otro"));
}

// ============================================================================
// 2. Exact 17 Owned Value Families
// ============================================================================

#[test]
fn owned_17_families_explicit_coverage() {
    let families: [OwnedValue; 17] = [
        OwnedValue::Boolean(true),
        OwnedValue::Int8(1),
        OwnedValue::Int16(2),
        OwnedValue::Int32(3),
        OwnedValue::Int64(4),
        OwnedValue::Int128(5),
        OwnedValue::Uint8(6),
        OwnedValue::Uint16(7),
        OwnedValue::Uint32(8),
        OwnedValue::Uint64(9),
        OwnedValue::Uint128(10),
        OwnedValue::Float32(11.0),
        OwnedValue::Float64(12.0),
        OwnedValue::String(Box::from("thirteen")),
        OwnedValue::Dynamic(OwnedDynamicValue::Float64(14.0)),
        OwnedValue::Struct(Box::new([OwnedValue::Int32(15)])),
        OwnedValue::Enum {
            variant: 16,
            payload: OwnedEnumPayload::Simple,
        },
    ];

    assert_eq!(families.len(), 17);
    assert_eq!(families[0], OwnedValue::Boolean(true));
    assert_eq!(families[13], OwnedValue::String(Box::from("thirteen")));
    assert_eq!(
        families[15],
        OwnedValue::Struct(Box::new([OwnedValue::Int32(15)]))
    );
}

#[test]
fn owned_boolean_variants() {
    assert_eq!(OwnedValue::Boolean(true), OwnedValue::Boolean(true));
    assert_ne!(OwnedValue::Boolean(true), OwnedValue::Boolean(false));
}

#[test]
fn owned_signed_integer_variants() {
    assert_eq!(OwnedValue::Int8(42), OwnedValue::Int8(42));
    assert_ne!(OwnedValue::Int8(42), OwnedValue::Int8(-42));

    assert_eq!(OwnedValue::Int16(1000), OwnedValue::Int16(1000));
    assert_ne!(OwnedValue::Int16(1000), OwnedValue::Int16(-1000));

    assert_eq!(OwnedValue::Int32(100_000), OwnedValue::Int32(100_000));
    assert_ne!(OwnedValue::Int32(100_000), OwnedValue::Int32(-100_000));

    assert_eq!(
        OwnedValue::Int64(10_000_000_000),
        OwnedValue::Int64(10_000_000_000)
    );
    assert_ne!(
        OwnedValue::Int64(10_000_000_000),
        OwnedValue::Int64(-10_000_000_000)
    );

    assert_eq!(
        OwnedValue::Int128(1_000_000_000_000_000_000_000),
        OwnedValue::Int128(1_000_000_000_000_000_000_000)
    );
    assert_ne!(
        OwnedValue::Int128(1_000_000_000_000_000_000_000),
        OwnedValue::Int128(-1_000_000_000_000_000_000_000)
    );
}

#[test]
fn owned_unsigned_integer_variants() {
    assert_eq!(OwnedValue::Uint8(255), OwnedValue::Uint8(255));
    assert_ne!(OwnedValue::Uint8(255), OwnedValue::Uint8(0));

    assert_eq!(OwnedValue::Uint16(65535), OwnedValue::Uint16(65535));
    assert_ne!(OwnedValue::Uint16(65535), OwnedValue::Uint16(0));

    assert_eq!(
        OwnedValue::Uint32(4_000_000_000),
        OwnedValue::Uint32(4_000_000_000)
    );
    assert_ne!(OwnedValue::Uint32(4_000_000_000), OwnedValue::Uint32(0));

    assert_eq!(
        OwnedValue::Uint64(18_000_000_000_000_000_000),
        OwnedValue::Uint64(18_000_000_000_000_000_000)
    );
    assert_ne!(
        OwnedValue::Uint64(18_000_000_000_000_000_000),
        OwnedValue::Uint64(0)
    );

    assert_eq!(
        OwnedValue::Uint128(300_000_000_000_000_000_000_000_000_000_000_000),
        OwnedValue::Uint128(300_000_000_000_000_000_000_000_000_000_000_000)
    );
    assert_ne!(
        OwnedValue::Uint128(300_000_000_000_000_000_000_000_000_000_000_000),
        OwnedValue::Uint128(0)
    );
}

#[test]
fn owned_float_variants() {
    assert_eq!(OwnedValue::Float32(1.25), OwnedValue::Float32(1.25));
    assert_ne!(OwnedValue::Float32(1.25), OwnedValue::Float32(2.5));

    assert_eq!(
        OwnedValue::Float64(123.456789012345),
        OwnedValue::Float64(123.456789012345)
    );
    assert_ne!(
        OwnedValue::Float64(123.456789012345),
        OwnedValue::Float64(987.654321098765)
    );
}

#[test]
fn owned_string_variant() {
    let s: Box<str> = Box::from("hola mundo");
    assert_eq!(
        OwnedValue::String(s.clone()),
        OwnedValue::String(Box::from("hola mundo"))
    );
    assert_ne!(OwnedValue::String(s), OwnedValue::String(Box::from("otro")));
}

// ============================================================================
// 3. Dynamic Integer — Canonical Forms
// ============================================================================

#[test]
fn dynamic_integer_canonical_zero() {
    // Borrowed canonical zero: negative = false, magnitude = []
    let borrowed_zero = DynamicIntegerValue {
        negative: false,
        magnitude: Cow::Borrowed(&[]),
    };
    assert!(!borrowed_zero.negative);
    assert!(borrowed_zero.magnitude.is_empty());

    let val_borrowed_zero = Value::Dynamic(DynamicValue::Integer(borrowed_zero));
    assert_eq!(
        val_borrowed_zero,
        Value::Dynamic(DynamicValue::Integer(DynamicIntegerValue {
            negative: false,
            magnitude: Cow::Borrowed(&[]),
        }))
    );

    // Owned canonical zero: negative = false, magnitude = []
    let owned_zero = OwnedDynamicInteger {
        negative: false,
        magnitude: Box::new([]),
    };
    assert!(!owned_zero.negative);
    assert!(owned_zero.magnitude.is_empty());

    let val_owned_zero = OwnedValue::Dynamic(OwnedDynamicValue::Integer(owned_zero));
    assert_eq!(
        val_owned_zero,
        OwnedValue::Dynamic(OwnedDynamicValue::Integer(OwnedDynamicInteger {
            negative: false,
            magnitude: Box::new([]),
        }))
    );
}

#[test]
fn dynamic_integer_positive_and_negative_magnitude() {
    // Positive big-endian magnitude
    let borrowed_pos = DynamicIntegerValue {
        negative: false,
        magnitude: Cow::Borrowed(&[0x01, 0x00]), // 256
    };
    let owned_pos = OwnedDynamicInteger {
        negative: false,
        magnitude: Box::new([0x01, 0x00]),
    };
    assert!(!borrowed_pos.negative);
    assert_eq!(borrowed_pos.magnitude.as_ref(), &[0x01, 0x00]);
    assert!(!owned_pos.negative);
    assert_eq!(owned_pos.magnitude.as_ref(), &[0x01, 0x00]);

    // Negative big-endian magnitude
    let borrowed_neg = DynamicIntegerValue {
        negative: true,
        magnitude: Cow::Borrowed(&[0x01, 0x00]), // -256
    };
    let owned_neg = OwnedDynamicInteger {
        negative: true,
        magnitude: Box::new([0x01, 0x00]),
    };
    assert!(borrowed_neg.negative);
    assert_eq!(borrowed_neg.magnitude.as_ref(), &[0x01, 0x00]);
    assert!(owned_neg.negative);
    assert_eq!(owned_neg.magnitude.as_ref(), &[0x01, 0x00]);

    // Distinctions
    assert_ne!(borrowed_pos, borrowed_neg);
    assert_ne!(owned_pos, owned_neg);
}

#[test]
fn dynamic_float_variants() {
    let dyn_f32 = Value::Dynamic(DynamicValue::Float32(1.5));
    let dyn_f64 = Value::Dynamic(DynamicValue::Float64(2.5));
    assert_eq!(dyn_f32, Value::Dynamic(DynamicValue::Float32(1.5)));
    assert_ne!(dyn_f32, Value::Dynamic(DynamicValue::Float32(3.5)));
    assert_eq!(dyn_f64, Value::Dynamic(DynamicValue::Float64(2.5)));

    let owned_dyn_f32 = OwnedValue::Dynamic(OwnedDynamicValue::Float32(1.5));
    let owned_dyn_f64 = OwnedValue::Dynamic(OwnedDynamicValue::Float64(2.5));
    assert_eq!(
        owned_dyn_f32,
        OwnedValue::Dynamic(OwnedDynamicValue::Float32(1.5))
    );
    assert_ne!(
        owned_dyn_f32,
        OwnedValue::Dynamic(OwnedDynamicValue::Float32(3.5))
    );
    assert_eq!(
        owned_dyn_f64,
        OwnedValue::Dynamic(OwnedDynamicValue::Float64(2.5))
    );
}

// ============================================================================
// 4. Nested Struct (Recursive, Order Preservation, Mixed Types)
// ============================================================================

#[test]
fn nested_struct_borrowed() {
    let nested_struct = Value::Struct(Box::new([
        Value::Int32(100),
        Value::Struct(Box::new([
            Value::String("inner text"),
            Value::Boolean(true),
        ])),
        Value::Enum {
            variant: 1,
            payload: EnumPayload::Simple,
        },
    ]));

    let expected = Value::Struct(Box::new([
        Value::Int32(100),
        Value::Struct(Box::new([
            Value::String("inner text"),
            Value::Boolean(true),
        ])),
        Value::Enum {
            variant: 1,
            payload: EnumPayload::Simple,
        },
    ]));

    let diff_inner_order = Value::Struct(Box::new([
        Value::Int32(100),
        Value::Struct(Box::new([
            Value::Boolean(true),
            Value::String("inner text"),
        ])),
        Value::Enum {
            variant: 1,
            payload: EnumPayload::Simple,
        },
    ]));

    assert_eq!(nested_struct, expected);
    assert_ne!(nested_struct, diff_inner_order);
}

#[test]
fn nested_struct_owned() {
    let nested_struct = OwnedValue::Struct(Box::new([
        OwnedValue::Int32(100),
        OwnedValue::Struct(Box::new([
            OwnedValue::String(Box::from("inner text")),
            OwnedValue::Boolean(true),
        ])),
        OwnedValue::Enum {
            variant: 1,
            payload: OwnedEnumPayload::Simple,
        },
    ]));

    let expected = OwnedValue::Struct(Box::new([
        OwnedValue::Int32(100),
        OwnedValue::Struct(Box::new([
            OwnedValue::String(Box::from("inner text")),
            OwnedValue::Boolean(true),
        ])),
        OwnedValue::Enum {
            variant: 1,
            payload: OwnedEnumPayload::Simple,
        },
    ]));

    let diff_inner_order = OwnedValue::Struct(Box::new([
        OwnedValue::Int32(100),
        OwnedValue::Struct(Box::new([
            OwnedValue::Boolean(true),
            OwnedValue::String(Box::from("inner text")),
        ])),
        OwnedValue::Enum {
            variant: 1,
            payload: OwnedEnumPayload::Simple,
        },
    ]));

    assert_eq!(nested_struct, expected);
    assert_ne!(nested_struct, diff_inner_order);
}

// ============================================================================
// 5. Enum (Simple, Associated, Structured)
// ============================================================================

#[test]
fn enum_payload_forms_borrowed() {
    // 1. Simple
    let simple = Value::Enum {
        variant: 0,
        payload: EnumPayload::Simple,
    };
    assert_eq!(
        simple,
        Value::Enum {
            variant: 0,
            payload: EnumPayload::Simple,
        }
    );
    assert_ne!(
        simple,
        Value::Enum {
            variant: 1,
            payload: EnumPayload::Simple,
        }
    );

    // 2. Associated
    let associated = Value::Enum {
        variant: 1,
        payload: EnumPayload::Associated(Box::new(Value::String("error message"))),
    };
    assert_eq!(
        associated,
        Value::Enum {
            variant: 1,
            payload: EnumPayload::Associated(Box::new(Value::String("error message"))),
        }
    );
    assert_ne!(
        associated,
        Value::Enum {
            variant: 1,
            payload: EnumPayload::Associated(Box::new(Value::String("different message"))),
        }
    );

    // 3. Structured
    let structured = Value::Enum {
        variant: 2,
        payload: EnumPayload::Structured {
            fields: Box::new([Value::Int32(1), Value::String("two"), Value::Boolean(false)]),
        },
    };
    let structured_same = Value::Enum {
        variant: 2,
        payload: EnumPayload::Structured {
            fields: Box::new([Value::Int32(1), Value::String("two"), Value::Boolean(false)]),
        },
    };
    let structured_diff_order = Value::Enum {
        variant: 2,
        payload: EnumPayload::Structured {
            fields: Box::new([Value::String("two"), Value::Int32(1), Value::Boolean(false)]),
        },
    };
    assert_eq!(structured, structured_same);
    assert_ne!(structured, structured_diff_order);
}

#[test]
fn enum_payload_forms_owned() {
    // 1. Simple
    let simple = OwnedValue::Enum {
        variant: 0,
        payload: OwnedEnumPayload::Simple,
    };
    assert_eq!(
        simple,
        OwnedValue::Enum {
            variant: 0,
            payload: OwnedEnumPayload::Simple,
        }
    );
    assert_ne!(
        simple,
        OwnedValue::Enum {
            variant: 1,
            payload: OwnedEnumPayload::Simple,
        }
    );

    // 2. Associated
    let associated = OwnedValue::Enum {
        variant: 1,
        payload: OwnedEnumPayload::Associated(Box::new(OwnedValue::String(Box::from(
            "error message",
        )))),
    };
    assert_eq!(
        associated,
        OwnedValue::Enum {
            variant: 1,
            payload: OwnedEnumPayload::Associated(Box::new(OwnedValue::String(Box::from(
                "error message",
            )))),
        }
    );
    assert_ne!(
        associated,
        OwnedValue::Enum {
            variant: 1,
            payload: OwnedEnumPayload::Associated(Box::new(OwnedValue::String(Box::from(
                "different message",
            )))),
        }
    );

    // 3. Structured
    let structured = OwnedValue::Enum {
        variant: 2,
        payload: OwnedEnumPayload::Structured {
            fields: Box::new([
                OwnedValue::Int32(1),
                OwnedValue::String(Box::from("two")),
                OwnedValue::Boolean(false),
            ]),
        },
    };
    let structured_same = OwnedValue::Enum {
        variant: 2,
        payload: OwnedEnumPayload::Structured {
            fields: Box::new([
                OwnedValue::Int32(1),
                OwnedValue::String(Box::from("two")),
                OwnedValue::Boolean(false),
            ]),
        },
    };
    let structured_diff_order = OwnedValue::Enum {
        variant: 2,
        payload: OwnedEnumPayload::Structured {
            fields: Box::new([
                OwnedValue::String(Box::from("two")),
                OwnedValue::Int32(1),
                OwnedValue::Boolean(false),
            ]),
        },
    };
    assert_eq!(structured, structured_same);
    assert_ne!(structured, structured_diff_order);
}

// ============================================================================
// 6. Borrowed String Lifetime
// ============================================================================

#[test]
fn borrowed_string_lifetime_bound_to_owner() {
    let owner_string = AllocString::from("local dynamically allocated string");
    let borrowed_slice: &str = owner_string.as_str();

    // Value::String borrows directly from owner_string without leak or 'static
    let value = Value::String(borrowed_slice);

    match value {
        Value::String(s) => {
            assert_eq!(s, "local dynamically allocated string");
            assert_eq!(s.len(), 34);
        }
        _ => panic!("expected Value::String variant"),
    }

    assert_eq!(value, Value::String("local dynamically allocated string"));
}

// ============================================================================
// 7. OwnedValue Autonomy
// ============================================================================

#[test]
fn owned_value_autonomy_after_source_scope() {
    let autonomous_value: OwnedValue = {
        let temp_src = AllocString::from("temporary string in nested block");
        let temp_elements = vec![
            OwnedValue::Int32(999),
            OwnedValue::String(Box::from(temp_src.as_str())),
            OwnedValue::Boolean(true),
        ];
        // temp_src and temp_elements are dropped when this block terminates
        OwnedValue::Struct(temp_elements.into_boxed_slice())
    };

    // autonomous_value outlives the source scope and remains completely valid
    match autonomous_value {
        OwnedValue::Struct(fields) => {
            assert_eq!(fields.len(), 3);
            assert_eq!(fields[0], OwnedValue::Int32(999));
            assert_eq!(
                fields[1],
                OwnedValue::String(Box::from("temporary string in nested block"))
            );
            assert_eq!(fields[2], OwnedValue::Boolean(true));
        }
        _ => panic!("expected OwnedValue::Struct variant"),
    }
}

// ============================================================================
// 8. Public Exports Access
// ============================================================================

#[test]
fn public_exports_access() {
    let _: evo_values::Value = evo_values::Value::Boolean(true);
    let _: evo_values::DynamicValue = evo_values::DynamicValue::Float32(1.0);
    let _: evo_values::DynamicIntegerValue = evo_values::DynamicIntegerValue {
        negative: false,
        magnitude: Cow::Borrowed(&[]),
    };
    let _: evo_values::EnumPayload = evo_values::EnumPayload::Simple;

    let _: evo_values::OwnedValue = evo_values::OwnedValue::Boolean(true);
    let _: evo_values::OwnedDynamicValue = evo_values::OwnedDynamicValue::Float32(1.0);
    let _: evo_values::OwnedDynamicInteger = evo_values::OwnedDynamicInteger {
        negative: false,
        magnitude: Box::new([]),
    };
    let _: evo_values::OwnedEnumPayload = evo_values::OwnedEnumPayload::Simple;

    let _: evo_values::definitions::Value = evo_values::definitions::Value::Boolean(false);
    let _: evo_values::definitions::OwnedValue =
        evo_values::definitions::OwnedValue::Boolean(false);
}
