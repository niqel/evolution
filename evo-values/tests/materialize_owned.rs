extern crate alloc;

use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::string::String as AllocString;
use alloc::vec;
use evo_values::{
    DynamicIntegerValue, DynamicValue, EnumPayload, MATERIALIZE_OWNED, MaterializeOwned,
    OwnedDynamicValue, OwnedEnumPayload, OwnedValue, Value, materialize_owned,
};

// ============================================================================
// 1. Function Pointer Contract Verification
// ============================================================================

#[test]
fn function_pointer_contract() {
    let op: MaterializeOwned = MATERIALIZE_OWNED;
    let input = Value::Boolean(true);
    let output = op(&input);
    assert!(matches!(output, OwnedValue::Boolean(true)));
}

// ============================================================================
// 2. All 17 Semantic Value Families
// ============================================================================

#[test]
fn materialize_17_families_coverage() {
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

    let op: MaterializeOwned = MATERIALIZE_OWNED;

    assert!(matches!(op(&families[0]), OwnedValue::Boolean(true)));
    assert!(matches!(op(&families[1]), OwnedValue::Int8(1)));
    assert!(matches!(op(&families[2]), OwnedValue::Int16(2)));
    assert!(matches!(op(&families[3]), OwnedValue::Int32(3)));
    assert!(matches!(op(&families[4]), OwnedValue::Int64(4)));
    assert!(matches!(op(&families[5]), OwnedValue::Int128(5)));
    assert!(matches!(op(&families[6]), OwnedValue::Uint8(6)));
    assert!(matches!(op(&families[7]), OwnedValue::Uint16(7)));
    assert!(matches!(op(&families[8]), OwnedValue::Uint32(8)));
    assert!(matches!(op(&families[9]), OwnedValue::Uint64(9)));
    assert!(matches!(op(&families[10]), OwnedValue::Uint128(10)));
    assert!(matches!(op(&families[11]), OwnedValue::Float32(f) if f == 11.0));
    assert!(matches!(op(&families[12]), OwnedValue::Float64(f) if f == 12.0));
    assert!(matches!(&op(&families[13]), OwnedValue::String(s) if &**s == "thirteen"));
    assert!(matches!(
        &op(&families[14]),
        OwnedValue::Dynamic(OwnedDynamicValue::Float64(f)) if *f == 14.0
    ));
    assert!(matches!(
        &op(&families[15]),
        OwnedValue::Struct(fields) if fields.len() == 1 && matches!(fields[0], OwnedValue::Int32(15))
    ));
    assert!(matches!(
        &op(&families[16]),
        OwnedValue::Enum {
            variant: 16,
            payload: OwnedEnumPayload::Simple
        }
    ));
}

#[test]
fn materialize_boolean_variants() {
    assert!(matches!(
        materialize_owned(&Value::Boolean(true)),
        OwnedValue::Boolean(true)
    ));
    assert!(matches!(
        materialize_owned(&Value::Boolean(false)),
        OwnedValue::Boolean(false)
    ));
}

#[test]
fn materialize_5_signed_integer_variants() {
    assert!(matches!(
        materialize_owned(&Value::Int8(-8)),
        OwnedValue::Int8(-8)
    ));
    assert!(matches!(
        materialize_owned(&Value::Int16(-1600)),
        OwnedValue::Int16(-1600)
    ));
    assert!(matches!(
        materialize_owned(&Value::Int32(-320_000)),
        OwnedValue::Int32(-320_000)
    ));
    assert!(matches!(
        materialize_owned(&Value::Int64(-64_000_000_000)),
        OwnedValue::Int64(-64_000_000_000)
    ));
    assert!(matches!(
        materialize_owned(&Value::Int128(-128_000_000_000_000_000_000_000)),
        OwnedValue::Int128(-128_000_000_000_000_000_000_000)
    ));
}

#[test]
fn materialize_5_unsigned_integer_variants() {
    assert!(matches!(
        materialize_owned(&Value::Uint8(8)),
        OwnedValue::Uint8(8)
    ));
    assert!(matches!(
        materialize_owned(&Value::Uint16(1600)),
        OwnedValue::Uint16(1600)
    ));
    assert!(matches!(
        materialize_owned(&Value::Uint32(320_000)),
        OwnedValue::Uint32(320_000)
    ));
    assert!(matches!(
        materialize_owned(&Value::Uint64(64_000_000_000)),
        OwnedValue::Uint64(64_000_000_000)
    ));
    assert!(matches!(
        materialize_owned(&Value::Uint128(128_000_000_000_000_000_000_000)),
        OwnedValue::Uint128(128_000_000_000_000_000_000_000)
    ));
}

#[test]
fn materialize_float_variants() {
    assert!(matches!(
        materialize_owned(&Value::Float32(3.14)),
        OwnedValue::Float32(f) if f == 3.14
    ));
    assert!(matches!(
        materialize_owned(&Value::Float64(2.718281828459045)),
        OwnedValue::Float64(f) if f == 2.718281828459045
    ));
}

// ============================================================================
// 3. String Autonomy
// ============================================================================

#[test]
fn materialize_string_content_and_autonomy() {
    let owned = {
        let temp = AllocString::from("autonomous heap string");
        materialize_owned(&Value::String(temp.as_str()))
    };

    match owned {
        OwnedValue::String(s) => {
            assert_eq!(&*s, "autonomous heap string");
        }
        _ => panic!("expected OwnedValue::String"),
    }
}

// ============================================================================
// 4. Dynamic Values (Integer & Float)
// ============================================================================

#[test]
fn materialize_dynamic_integer_cases() {
    // Positive
    let pos_int = DynamicIntegerValue::from_parts(false, Cow::Borrowed(&[0x01, 0x02]));
    let pos_owned = materialize_owned(&Value::Dynamic(DynamicValue::Integer(pos_int)));
    match pos_owned {
        OwnedValue::Dynamic(OwnedDynamicValue::Integer(i)) => {
            assert!(!i.negative());
            assert_eq!(i.magnitude(), &[0x01, 0x02]);
        }
        _ => panic!("expected OwnedDynamicValue::Integer"),
    }

    // Negative
    let neg_int = DynamicIntegerValue::from_parts(true, Cow::Borrowed(&[0x05]));
    let neg_owned = materialize_owned(&Value::Dynamic(DynamicValue::Integer(neg_int)));
    match neg_owned {
        OwnedValue::Dynamic(OwnedDynamicValue::Integer(i)) => {
            assert!(i.negative());
            assert_eq!(i.magnitude(), &[0x05]);
        }
        _ => panic!("expected OwnedDynamicValue::Integer"),
    }

    // Zero
    let zero_int = DynamicIntegerValue::from_parts(true, Cow::Borrowed(&[]));
    let zero_owned = materialize_owned(&Value::Dynamic(DynamicValue::Integer(zero_int)));
    match zero_owned {
        OwnedValue::Dynamic(OwnedDynamicValue::Integer(i)) => {
            assert!(!i.negative());
            assert_eq!(i.magnitude(), &[]);
        }
        _ => panic!("expected OwnedDynamicValue::Integer"),
    }

    // Multibyte magnitude with autonomy
    let owned_multibyte = {
        let temp_bytes = vec![0x01, 0x00, 0x00, 0xFF];
        let borrowed = DynamicIntegerValue::from_parts(false, Cow::Borrowed(&temp_bytes));
        materialize_owned(&Value::Dynamic(DynamicValue::Integer(borrowed)))
    };
    match owned_multibyte {
        OwnedValue::Dynamic(OwnedDynamicValue::Integer(i)) => {
            assert!(!i.negative());
            assert_eq!(i.magnitude(), &[0x01, 0x00, 0x00, 0xFF]);
        }
        _ => panic!("expected OwnedDynamicValue::Integer"),
    }
}

#[test]
fn materialize_dynamic_float_variants() {
    let dyn_f32 = Value::Dynamic(DynamicValue::Float32(1.5));
    let owned_f32 = materialize_owned(&dyn_f32);
    assert!(matches!(
        owned_f32,
        OwnedValue::Dynamic(OwnedDynamicValue::Float32(f)) if f == 1.5
    ));

    let dyn_f64 = Value::Dynamic(DynamicValue::Float64(2.5));
    let owned_f64 = materialize_owned(&dyn_f64);
    assert!(matches!(
        owned_f64,
        OwnedValue::Dynamic(OwnedDynamicValue::Float64(f)) if f == 2.5
    ));
}

// ============================================================================
// 5. Struct (Recursive, Order Preservation, Mixed Types)
// ============================================================================

#[test]
fn materialize_struct_mixed_types_and_order_preservation() {
    let borrowed_struct = Value::Struct(Box::new([
        Value::Int32(10),
        Value::String("second"),
        Value::Dynamic(DynamicValue::Float64(30.0)),
        Value::Boolean(false),
    ]));

    let owned = materialize_owned(&borrowed_struct);
    match owned {
        OwnedValue::Struct(fields) => {
            assert_eq!(fields.len(), 4);
            assert!(matches!(fields[0], OwnedValue::Int32(10)));
            assert!(matches!(&fields[1], OwnedValue::String(s) if &**s == "second"));
            assert!(matches!(
                &fields[2],
                OwnedValue::Dynamic(OwnedDynamicValue::Float64(f)) if *f == 30.0
            ));
            assert!(matches!(fields[3], OwnedValue::Boolean(false)));
        }
        _ => panic!("expected OwnedValue::Struct"),
    }
}

// ============================================================================
// 6. Enum (Simple, Associated, Structured)
// ============================================================================

#[test]
fn materialize_enum_simple() {
    let borrowed = Value::Enum {
        variant: 7,
        payload: EnumPayload::Simple,
    };
    let owned = materialize_owned(&borrowed);
    assert!(matches!(
        owned,
        OwnedValue::Enum {
            variant: 7,
            payload: OwnedEnumPayload::Simple,
        }
    ));
}

#[test]
fn materialize_enum_associated() {
    let borrowed = Value::Enum {
        variant: 3,
        payload: EnumPayload::Associated(Box::new(Value::String("associated inner"))),
    };
    let owned = materialize_owned(&borrowed);
    match owned {
        OwnedValue::Enum { variant, payload } => {
            assert_eq!(variant, 3);
            match payload {
                OwnedEnumPayload::Associated(inner) => {
                    assert!(matches!(&*inner, OwnedValue::String(s) if &**s == "associated inner"));
                }
                _ => panic!("expected Associated payload"),
            }
        }
        _ => panic!("expected Enum"),
    }
}

#[test]
fn materialize_enum_structured() {
    let borrowed = Value::Enum {
        variant: 5,
        payload: EnumPayload::Structured {
            fields: Box::new([Value::Int64(100), Value::Boolean(true)]),
        },
    };
    let owned = materialize_owned(&borrowed);
    match owned {
        OwnedValue::Enum { variant, payload } => {
            assert_eq!(variant, 5);
            match payload {
                OwnedEnumPayload::Structured { fields } => {
                    assert_eq!(fields.len(), 2);
                    assert!(matches!(fields[0], OwnedValue::Int64(100)));
                    assert!(matches!(fields[1], OwnedValue::Boolean(true)));
                }
                _ => panic!("expected Structured payload"),
            }
        }
        _ => panic!("expected Enum"),
    }
}

// ============================================================================
// 7. Deep Recursive Composite Tree
// ============================================================================

#[test]
fn materialize_deep_recursive_tree() {
    // Tree structure:
    // Struct
    // └── Enum
    //     └── Associated
    //         └── Struct
    //             └── String
    let borrowed_tree = Value::Struct(Box::new([Value::Enum {
        variant: 42,
        payload: EnumPayload::Associated(Box::new(Value::Struct(Box::new([Value::String(
            "deeply nested string",
        )])))),
    }]));

    let owned_tree = materialize_owned(&borrowed_tree);

    match owned_tree {
        OwnedValue::Struct(outer_fields) => {
            assert_eq!(outer_fields.len(), 1);
            match &outer_fields[0] {
                OwnedValue::Enum { variant, payload } => {
                    assert_eq!(*variant, 42);
                    match payload {
                        OwnedEnumPayload::Associated(inner_val) => match &**inner_val {
                            OwnedValue::Struct(inner_fields) => {
                                assert_eq!(inner_fields.len(), 1);
                                assert!(matches!(
                                    &inner_fields[0],
                                    OwnedValue::String(s) if &**s == "deeply nested string"
                                ));
                            }
                            _ => panic!("expected inner OwnedValue::Struct"),
                        },
                        _ => panic!("expected Associated payload"),
                    }
                }
                _ => panic!("expected OwnedValue::Enum"),
            }
        }
        _ => panic!("expected outer OwnedValue::Struct"),
    }
}
