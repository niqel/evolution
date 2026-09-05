extern crate alloc;

use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::string::String as AllocString;
use alloc::vec;
use evo_values::{
    BitwiseFailure, ComparisonFailure, ConversionFailure, DynamicIntegerValue, DynamicValue,
    EnumPayload, NumericFailure, OwnedDynamicInteger, OwnedDynamicValue, OwnedEnumPayload,
    OwnedValue, PowerExponent, ProductionControl, ShiftAmount, TextLength, TextOperationFailure,
    TextPosition, Value,
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
    assert!(matches!(families[0], Value::Boolean(true)));
    assert!(matches!(families[1], Value::Int8(1)));
    assert!(matches!(families[2], Value::Int16(2)));
    assert!(matches!(families[3], Value::Int32(3)));
    assert!(matches!(families[4], Value::Int64(4)));
    assert!(matches!(families[5], Value::Int128(5)));
    assert!(matches!(families[6], Value::Uint8(6)));
    assert!(matches!(families[7], Value::Uint16(7)));
    assert!(matches!(families[8], Value::Uint32(8)));
    assert!(matches!(families[9], Value::Uint64(9)));
    assert!(matches!(families[10], Value::Uint128(10)));
    assert!(matches!(families[11], Value::Float32(f) if f == 11.0));
    assert!(matches!(families[12], Value::Float64(f) if f == 12.0));
    assert!(matches!(families[13], Value::String("thirteen")));
    assert!(matches!(
        families[14],
        Value::Dynamic(DynamicValue::Float64(f)) if f == 14.0
    ));
    assert!(matches!(
        &families[15],
        Value::Struct(fields) if fields.len() == 1 && matches!(fields[0], Value::Int32(15))
    ));
    assert!(matches!(
        &families[16],
        Value::Enum {
            variant: 16,
            payload: EnumPayload::Simple
        }
    ));
}

#[test]
fn borrowed_boolean_variants() {
    let t = Value::Boolean(true);
    let f = Value::Boolean(false);
    assert!(matches!(t, Value::Boolean(true)));
    assert!(matches!(f, Value::Boolean(false)));
}

#[test]
fn borrowed_signed_integer_variants() {
    assert!(matches!(Value::Int8(42), Value::Int8(42)));
    assert!(matches!(Value::Int8(-42), Value::Int8(-42)));

    assert!(matches!(Value::Int16(1000), Value::Int16(1000)));
    assert!(matches!(Value::Int16(-1000), Value::Int16(-1000)));

    assert!(matches!(Value::Int32(100_000), Value::Int32(100_000)));
    assert!(matches!(Value::Int32(-100_000), Value::Int32(-100_000)));

    assert!(matches!(
        Value::Int64(10_000_000_000),
        Value::Int64(10_000_000_000)
    ));
    assert!(matches!(
        Value::Int64(-10_000_000_000),
        Value::Int64(-10_000_000_000)
    ));

    assert!(matches!(
        Value::Int128(1_000_000_000_000_000_000_000),
        Value::Int128(1_000_000_000_000_000_000_000)
    ));
    assert!(matches!(
        Value::Int128(-1_000_000_000_000_000_000_000),
        Value::Int128(-1_000_000_000_000_000_000_000)
    ));
}

#[test]
fn borrowed_unsigned_integer_variants() {
    assert!(matches!(Value::Uint8(255), Value::Uint8(255)));
    assert!(matches!(Value::Uint8(0), Value::Uint8(0)));

    assert!(matches!(Value::Uint16(65535), Value::Uint16(65535)));
    assert!(matches!(Value::Uint16(0), Value::Uint16(0)));

    assert!(matches!(
        Value::Uint32(4_000_000_000),
        Value::Uint32(4_000_000_000)
    ));
    assert!(matches!(Value::Uint32(0), Value::Uint32(0)));

    assert!(matches!(
        Value::Uint64(18_000_000_000_000_000_000),
        Value::Uint64(18_000_000_000_000_000_000)
    ));
    assert!(matches!(Value::Uint64(0), Value::Uint64(0)));

    assert!(matches!(
        Value::Uint128(300_000_000_000_000_000_000_000_000_000_000_000),
        Value::Uint128(300_000_000_000_000_000_000_000_000_000_000_000)
    ));
    assert!(matches!(Value::Uint128(0), Value::Uint128(0)));
}

#[test]
fn borrowed_float_variants() {
    assert!(matches!(Value::Float32(1.25), Value::Float32(f) if f == 1.25));
    assert!(matches!(Value::Float64(123.456789012345), Value::Float64(f) if f == 123.456789012345));
}

#[test]
fn borrowed_string_variant() {
    let text = "hola mundo";
    assert!(matches!(Value::String(text), Value::String("hola mundo")));
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
    assert!(matches!(families[0], OwnedValue::Boolean(true)));
    assert!(matches!(families[1], OwnedValue::Int8(1)));
    assert!(matches!(families[2], OwnedValue::Int16(2)));
    assert!(matches!(families[3], OwnedValue::Int32(3)));
    assert!(matches!(families[4], OwnedValue::Int64(4)));
    assert!(matches!(families[5], OwnedValue::Int128(5)));
    assert!(matches!(families[6], OwnedValue::Uint8(6)));
    assert!(matches!(families[7], OwnedValue::Uint16(7)));
    assert!(matches!(families[8], OwnedValue::Uint32(8)));
    assert!(matches!(families[9], OwnedValue::Uint64(9)));
    assert!(matches!(families[10], OwnedValue::Uint128(10)));
    assert!(matches!(families[11], OwnedValue::Float32(f) if f == 11.0));
    assert!(matches!(families[12], OwnedValue::Float64(f) if f == 12.0));
    assert!(matches!(&families[13], OwnedValue::String(s) if &**s == "thirteen"));
    assert!(matches!(
        &families[14],
        OwnedValue::Dynamic(OwnedDynamicValue::Float64(f)) if *f == 14.0
    ));
    assert!(matches!(
        &families[15],
        OwnedValue::Struct(fields) if fields.len() == 1 && matches!(fields[0], OwnedValue::Int32(15))
    ));
    assert!(matches!(
        &families[16],
        OwnedValue::Enum {
            variant: 16,
            payload: OwnedEnumPayload::Simple
        }
    ));
}

#[test]
fn owned_boolean_variants() {
    let t = OwnedValue::Boolean(true);
    let f = OwnedValue::Boolean(false);
    assert!(matches!(t, OwnedValue::Boolean(true)));
    assert!(matches!(f, OwnedValue::Boolean(false)));
}

#[test]
fn owned_signed_integer_variants() {
    assert!(matches!(OwnedValue::Int8(42), OwnedValue::Int8(42)));
    assert!(matches!(OwnedValue::Int8(-42), OwnedValue::Int8(-42)));

    assert!(matches!(OwnedValue::Int16(1000), OwnedValue::Int16(1000)));
    assert!(matches!(OwnedValue::Int16(-1000), OwnedValue::Int16(-1000)));

    assert!(matches!(
        OwnedValue::Int32(100_000),
        OwnedValue::Int32(100_000)
    ));
    assert!(matches!(
        OwnedValue::Int32(-100_000),
        OwnedValue::Int32(-100_000)
    ));

    assert!(matches!(
        OwnedValue::Int64(10_000_000_000),
        OwnedValue::Int64(10_000_000_000)
    ));
    assert!(matches!(
        OwnedValue::Int64(-10_000_000_000),
        OwnedValue::Int64(-10_000_000_000)
    ));

    assert!(matches!(
        OwnedValue::Int128(1_000_000_000_000_000_000_000),
        OwnedValue::Int128(1_000_000_000_000_000_000_000)
    ));
    assert!(matches!(
        OwnedValue::Int128(-1_000_000_000_000_000_000_000),
        OwnedValue::Int128(-1_000_000_000_000_000_000_000)
    ));
}

#[test]
fn owned_unsigned_integer_variants() {
    assert!(matches!(OwnedValue::Uint8(255), OwnedValue::Uint8(255)));
    assert!(matches!(OwnedValue::Uint8(0), OwnedValue::Uint8(0)));

    assert!(matches!(
        OwnedValue::Uint16(65535),
        OwnedValue::Uint16(65535)
    ));
    assert!(matches!(OwnedValue::Uint16(0), OwnedValue::Uint16(0)));

    assert!(matches!(
        OwnedValue::Uint32(4_000_000_000),
        OwnedValue::Uint32(4_000_000_000)
    ));
    assert!(matches!(OwnedValue::Uint32(0), OwnedValue::Uint32(0)));

    assert!(matches!(
        OwnedValue::Uint64(18_000_000_000_000_000_000),
        OwnedValue::Uint64(18_000_000_000_000_000_000)
    ));
    assert!(matches!(OwnedValue::Uint64(0), OwnedValue::Uint64(0)));

    assert!(matches!(
        OwnedValue::Uint128(300_000_000_000_000_000_000_000_000_000_000_000),
        OwnedValue::Uint128(300_000_000_000_000_000_000_000_000_000_000_000)
    ));
    assert!(matches!(OwnedValue::Uint128(0), OwnedValue::Uint128(0)));
}

#[test]
fn owned_float_variants() {
    assert!(matches!(OwnedValue::Float32(1.25), OwnedValue::Float32(f) if f == 1.25));
    assert!(
        matches!(OwnedValue::Float64(123.456789012345), OwnedValue::Float64(f) if f == 123.456789012345)
    );
}

#[test]
fn owned_string_variant() {
    let s: Box<str> = Box::from("hola mundo");
    assert!(
        matches!(&OwnedValue::String(s), OwnedValue::String(inner) if &**inner == "hola mundo")
    );
}

// ============================================================================
// 3. Dynamic Values & OwnedDynamicValue Availability
// ============================================================================

#[test]
fn dynamic_float_variants() {
    let dyn_f32 = Value::Dynamic(DynamicValue::Float32(1.5));
    let dyn_f64 = Value::Dynamic(DynamicValue::Float64(2.5));
    assert!(matches!(dyn_f32, Value::Dynamic(DynamicValue::Float32(f)) if f == 1.5));
    assert!(matches!(dyn_f64, Value::Dynamic(DynamicValue::Float64(f)) if f == 2.5));

    let owned_dyn_f32 = OwnedValue::Dynamic(OwnedDynamicValue::Float32(1.5));
    let owned_dyn_f64 = OwnedValue::Dynamic(OwnedDynamicValue::Float64(2.5));
    assert!(matches!(
        owned_dyn_f32,
        OwnedValue::Dynamic(OwnedDynamicValue::Float32(f)) if f == 1.5
    ));
    assert!(matches!(
        owned_dyn_f64,
        OwnedValue::Dynamic(OwnedDynamicValue::Float64(f)) if f == 2.5
    ));
}

#[test]
fn owned_dynamic_value_available() {
    let val_f32 = OwnedDynamicValue::Float32(3.14);
    let val_f64 = OwnedDynamicValue::Float64(6.28);
    assert!(matches!(val_f32, OwnedDynamicValue::Float32(f) if f == 3.14));
    assert!(matches!(val_f64, OwnedDynamicValue::Float64(f) if f == 6.28));
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

    match nested_struct {
        Value::Struct(fields) => {
            assert_eq!(fields.len(), 3);
            assert!(matches!(fields[0], Value::Int32(100)));
            match &fields[1] {
                Value::Struct(inner_fields) => {
                    assert_eq!(inner_fields.len(), 2);
                    assert!(matches!(inner_fields[0], Value::String("inner text")));
                    assert!(matches!(inner_fields[1], Value::Boolean(true)));
                }
                _ => panic!("expected inner Struct"),
            }
            assert!(matches!(
                &fields[2],
                Value::Enum {
                    variant: 1,
                    payload: EnumPayload::Simple
                }
            ));
        }
        _ => panic!("expected outer Struct"),
    }
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

    match nested_struct {
        OwnedValue::Struct(fields) => {
            assert_eq!(fields.len(), 3);
            assert!(matches!(fields[0], OwnedValue::Int32(100)));
            match &fields[1] {
                OwnedValue::Struct(inner_fields) => {
                    assert_eq!(inner_fields.len(), 2);
                    assert!(
                        matches!(&inner_fields[0], OwnedValue::String(s) if &**s == "inner text")
                    );
                    assert!(matches!(inner_fields[1], OwnedValue::Boolean(true)));
                }
                _ => panic!("expected inner Struct"),
            }
            assert!(matches!(
                &fields[2],
                OwnedValue::Enum {
                    variant: 1,
                    payload: OwnedEnumPayload::Simple
                }
            ));
        }
        _ => panic!("expected outer Struct"),
    }
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
    assert!(matches!(
        simple,
        Value::Enum {
            variant: 0,
            payload: EnumPayload::Simple
        }
    ));

    // 2. Associated
    let associated = Value::Enum {
        variant: 1,
        payload: EnumPayload::Associated(Box::new(Value::String("error message"))),
    };
    match associated {
        Value::Enum { variant, payload } => {
            assert_eq!(variant, 1);
            match payload {
                EnumPayload::Associated(inner) => {
                    assert!(matches!(*inner, Value::String("error message")));
                }
                _ => panic!("expected Associated payload"),
            }
        }
        _ => panic!("expected Enum"),
    }

    // 3. Structured
    let structured = Value::Enum {
        variant: 2,
        payload: EnumPayload::Structured {
            fields: Box::new([Value::Int32(1), Value::String("two"), Value::Boolean(false)]),
        },
    };
    match structured {
        Value::Enum { variant, payload } => {
            assert_eq!(variant, 2);
            match payload {
                EnumPayload::Structured { fields } => {
                    assert_eq!(fields.len(), 3);
                    assert!(matches!(fields[0], Value::Int32(1)));
                    assert!(matches!(fields[1], Value::String("two")));
                    assert!(matches!(fields[2], Value::Boolean(false)));
                }
                _ => panic!("expected Structured payload"),
            }
        }
        _ => panic!("expected Enum"),
    }
}

#[test]
fn enum_payload_forms_owned() {
    // 1. Simple
    let simple = OwnedValue::Enum {
        variant: 0,
        payload: OwnedEnumPayload::Simple,
    };
    assert!(matches!(
        simple,
        OwnedValue::Enum {
            variant: 0,
            payload: OwnedEnumPayload::Simple
        }
    ));

    // 2. Associated
    let associated = OwnedValue::Enum {
        variant: 1,
        payload: OwnedEnumPayload::Associated(Box::new(OwnedValue::String(Box::from(
            "error message",
        )))),
    };
    match associated {
        OwnedValue::Enum { variant, payload } => {
            assert_eq!(variant, 1);
            match payload {
                OwnedEnumPayload::Associated(inner) => {
                    assert!(matches!(&*inner, OwnedValue::String(s) if &**s == "error message"));
                }
                _ => panic!("expected Associated payload"),
            }
        }
        _ => panic!("expected Enum"),
    }

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
    match structured {
        OwnedValue::Enum { variant, payload } => {
            assert_eq!(variant, 2);
            match payload {
                OwnedEnumPayload::Structured { fields } => {
                    assert_eq!(fields.len(), 3);
                    assert!(matches!(fields[0], OwnedValue::Int32(1)));
                    assert!(matches!(&fields[1], OwnedValue::String(s) if &**s == "two"));
                    assert!(matches!(fields[2], OwnedValue::Boolean(false)));
                }
                _ => panic!("expected Structured payload"),
            }
        }
        _ => panic!("expected Enum"),
    }
}

// ============================================================================
// 6. Borrowed String Lifetime
// ============================================================================

#[test]
fn borrowed_string_lifetime_bound_to_owner() {
    let owner_string = AllocString::from("local dynamically allocated string");
    let borrowed_slice: &str = owner_string.as_str();

    let value = Value::String(borrowed_slice);

    match value {
        Value::String(s) => {
            assert_eq!(s, "local dynamically allocated string");
            assert_eq!(s.len(), 34);
        }
        _ => panic!("expected Value::String variant"),
    }
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
        OwnedValue::Struct(temp_elements.into_boxed_slice())
    };

    match autonomous_value {
        OwnedValue::Struct(fields) => {
            assert_eq!(fields.len(), 3);
            assert!(matches!(fields[0], OwnedValue::Int32(999)));
            assert!(matches!(
                &fields[1],
                OwnedValue::String(s) if &**s == "temporary string in nested block"
            ));
            assert!(matches!(fields[2], OwnedValue::Boolean(true)));
        }
        _ => panic!("expected OwnedValue::Struct variant"),
    }
}

// ============================================================================
// 8. Value Tree Clone
// ============================================================================

#[test]
fn value_and_owned_value_clone() {
    let val = Value::Int32(42);
    let cloned_val = val.clone();
    assert!(matches!(cloned_val, Value::Int32(42)));

    let owned = OwnedValue::String(Box::from("test"));
    let cloned_owned = owned.clone();
    assert!(matches!(&cloned_owned, OwnedValue::String(s) if &**s == "test"));
}

// ============================================================================
// 9. Semantic Scalars (Newtypes)
// ============================================================================

#[test]
fn semantic_scalars_construction_and_equality() {
    let pos = TextPosition(42);
    let len = TextLength(100);
    let shift = ShiftAmount(5);
    let exp = PowerExponent(3);

    assert_eq!(pos.0, 42);
    assert_eq!(len.0, 100);
    assert_eq!(shift.0, 5);
    assert_eq!(exp.0, 3);

    assert_eq!(pos, TextPosition(42));
    assert_ne!(pos, TextPosition(0));

    assert_eq!(len, TextLength(100));
    assert_ne!(len, TextLength(0));

    assert_eq!(shift, ShiftAmount(5));
    assert_ne!(shift, ShiftAmount(0));

    assert_eq!(exp, PowerExponent(3));
    assert_ne!(exp, PowerExponent(0));

    // Test Copy and Clone
    let pos_copy = pos;
    assert_eq!(pos, pos_copy);
    let len_copy = len;
    assert_eq!(len, len_copy);
    let shift_copy = shift;
    assert_eq!(shift, shift_copy);
    let exp_copy = exp;
    assert_eq!(exp, exp_copy);
}

// ============================================================================
// 10. ProductionControl
// ============================================================================

#[test]
fn production_control_contains_continue_and_stop() {
    let c = ProductionControl::Continue;
    let s = ProductionControl::Stop;

    assert_eq!(c, ProductionControl::Continue);
    assert_eq!(s, ProductionControl::Stop);
    assert_ne!(c, s);

    // Test Copy and Clone
    let c_copy = c;
    assert_eq!(c, c_copy);
    let s_copy = s;
    assert_eq!(s, s_copy);
}

// ============================================================================
// 11. Failure Enums
// ============================================================================

#[test]
fn failure_enums_contain_closed_variants() {
    // TextOperationFailure: OutOfBounds, EmptyPattern, EmptySeparator
    let t_oob = TextOperationFailure::OutOfBounds;
    let t_ep = TextOperationFailure::EmptyPattern;
    let t_es = TextOperationFailure::EmptySeparator;
    assert_eq!(t_oob, TextOperationFailure::OutOfBounds);
    assert_eq!(t_ep, TextOperationFailure::EmptyPattern);
    assert_eq!(t_es, TextOperationFailure::EmptySeparator);
    assert_ne!(t_oob, t_ep);
    assert_ne!(t_ep, t_es);
    assert_ne!(t_oob, t_es);

    // NumericFailure: Overflow, DivisionByZero, InvalidBounds
    let n_of = NumericFailure::Overflow;
    let n_dbz = NumericFailure::DivisionByZero;
    let n_ib = NumericFailure::InvalidBounds;
    assert_eq!(n_of, NumericFailure::Overflow);
    assert_eq!(n_dbz, NumericFailure::DivisionByZero);
    assert_eq!(n_ib, NumericFailure::InvalidBounds);
    assert_ne!(n_of, n_dbz);
    assert_ne!(n_dbz, n_ib);
    assert_ne!(n_of, n_ib);

    // BitwiseFailure: InvalidShift
    let b_is = BitwiseFailure::InvalidShift;
    assert_eq!(b_is, BitwiseFailure::InvalidShift);

    // ComparisonFailure: DifferentFamily, NotComparable
    let c_df = ComparisonFailure::DifferentFamily;
    let c_nc = ComparisonFailure::NotComparable;
    assert_eq!(c_df, ComparisonFailure::DifferentFamily);
    assert_eq!(c_nc, ComparisonFailure::NotComparable);
    assert_ne!(c_df, c_nc);

    // ConversionFailure: NotExactlyRepresentable
    let cv_ner = ConversionFailure::NotExactlyRepresentable;
    assert_eq!(cv_ner, ConversionFailure::NotExactlyRepresentable);
}

// ============================================================================
// 12. Public Exports Access
// ============================================================================

#[test]
fn public_exports_access() {
    let _: evo_values::Value = evo_values::Value::Boolean(true);
    let _: evo_values::DynamicValue = evo_values::DynamicValue::Float32(1.0);
    let _: evo_values::EnumPayload = evo_values::EnumPayload::Simple;

    let _: evo_values::OwnedValue = evo_values::OwnedValue::Boolean(true);
    let _: evo_values::OwnedDynamicValue = evo_values::OwnedDynamicValue::Float32(1.0);
    let _: evo_values::OwnedEnumPayload = evo_values::OwnedEnumPayload::Simple;

    let _: evo_values::definitions::Value = evo_values::definitions::Value::Boolean(false);
    let _: evo_values::definitions::OwnedValue =
        evo_values::definitions::OwnedValue::Boolean(false);

    let _: evo_values::TextPosition = evo_values::TextPosition(0);
    let _: evo_values::TextLength = evo_values::TextLength(0);
    let _: evo_values::ShiftAmount = evo_values::ShiftAmount(0);
    let _: evo_values::PowerExponent = evo_values::PowerExponent(0);

    let _: evo_values::ProductionControl = evo_values::ProductionControl::Continue;

    let _: evo_values::TextOperationFailure = evo_values::TextOperationFailure::OutOfBounds;
    let _: evo_values::NumericFailure = evo_values::NumericFailure::Overflow;
    let _: evo_values::BitwiseFailure = evo_values::BitwiseFailure::InvalidShift;
    let _: evo_values::ComparisonFailure = evo_values::ComparisonFailure::DifferentFamily;
    let _: evo_values::ConversionFailure = evo_values::ConversionFailure::NotExactlyRepresentable;

    let dyn_int = evo_values::DynamicIntegerValue::from_parts(false, Cow::Borrowed(&[]));
    assert!(!dyn_int.negative());
    assert_eq!(dyn_int.magnitude(), &[]);

    let owned_dyn_int = evo_values::OwnedDynamicInteger::from_parts(false, Box::new([]));
    assert!(!owned_dyn_int.negative());
    assert_eq!(owned_dyn_int.magnitude(), &[]);
}

// ============================================================================
// 13. Canonical Dynamic Integer (Borrowed & Owned)
// ============================================================================

#[test]
fn borrowed_dynamic_integer_case_1_zero_normal() {
    let val = DynamicIntegerValue::from_parts(false, Cow::Borrowed(&[]));
    assert!(!val.negative());
    assert_eq!(val.magnitude(), &[]);
}

#[test]
fn borrowed_dynamic_integer_case_2_negative_zero() {
    let val = DynamicIntegerValue::from_parts(true, Cow::Borrowed(&[]));
    assert!(!val.negative());
    assert_eq!(val.magnitude(), &[]);
}

#[test]
fn borrowed_dynamic_integer_case_3_zero_single_zero_byte() {
    let val_pos = DynamicIntegerValue::from_parts(false, Cow::Borrowed(&[0x00]));
    assert!(!val_pos.negative());
    assert_eq!(val_pos.magnitude(), &[]);

    let val_neg = DynamicIntegerValue::from_parts(true, Cow::Borrowed(&[0x00]));
    assert!(!val_neg.negative());
    assert_eq!(val_neg.magnitude(), &[]);
}

#[test]
fn borrowed_dynamic_integer_case_4_zero_multiple_zero_bytes() {
    let val_pos = DynamicIntegerValue::from_parts(false, Cow::Borrowed(&[0x00, 0x00, 0x00]));
    assert!(!val_pos.negative());
    assert_eq!(val_pos.magnitude(), &[]);

    let val_neg = DynamicIntegerValue::from_parts(true, Cow::Borrowed(&[0x00, 0x00, 0x00]));
    assert!(!val_neg.negative());
    assert_eq!(val_neg.magnitude(), &[]);
}

#[test]
fn borrowed_dynamic_integer_case_5_positive_one() {
    let val = DynamicIntegerValue::from_parts(false, Cow::Borrowed(&[0x01]));
    assert!(!val.negative());
    assert_eq!(val.magnitude(), &[0x01]);
}

#[test]
fn borrowed_dynamic_integer_case_6_negative_one() {
    let val = DynamicIntegerValue::from_parts(true, Cow::Borrowed(&[0x01]));
    assert!(val.negative());
    assert_eq!(val.magnitude(), &[0x01]);
}

#[test]
fn borrowed_dynamic_integer_case_7_magnitude_without_redundant_zeros() {
    let val_pos = DynamicIntegerValue::from_parts(false, Cow::Borrowed(&[0x01, 0x02]));
    assert!(!val_pos.negative());
    assert_eq!(val_pos.magnitude(), &[0x01, 0x02]);

    let val_neg = DynamicIntegerValue::from_parts(true, Cow::Borrowed(&[0x01, 0x02]));
    assert!(val_neg.negative());
    assert_eq!(val_neg.magnitude(), &[0x01, 0x02]);
}

#[test]
fn borrowed_dynamic_integer_case_8_single_leading_zero() {
    let val_pos = DynamicIntegerValue::from_parts(false, Cow::Borrowed(&[0x00, 0x01]));
    assert!(!val_pos.negative());
    assert_eq!(val_pos.magnitude(), &[0x01]);

    let val_neg = DynamicIntegerValue::from_parts(true, Cow::Borrowed(&[0x00, 0x01]));
    assert!(val_neg.negative());
    assert_eq!(val_neg.magnitude(), &[0x01]);
}

#[test]
fn borrowed_dynamic_integer_case_9_multiple_leading_zeros() {
    let val_pos = DynamicIntegerValue::from_parts(false, Cow::Borrowed(&[0x00, 0x00, 0x01]));
    assert!(!val_pos.negative());
    assert_eq!(val_pos.magnitude(), &[0x01]);

    let val_neg = DynamicIntegerValue::from_parts(true, Cow::Borrowed(&[0x00, 0x00, 0x01]));
    assert!(val_neg.negative());
    assert_eq!(val_neg.magnitude(), &[0x01]);
}

#[test]
fn borrowed_dynamic_integer_case_10_significant_internal_zero() {
    let val_pos = DynamicIntegerValue::from_parts(false, Cow::Borrowed(&[0x01, 0x00, 0x02]));
    assert!(!val_pos.negative());
    assert_eq!(val_pos.magnitude(), &[0x01, 0x00, 0x02]);

    let val_neg = DynamicIntegerValue::from_parts(true, Cow::Borrowed(&[0x01, 0x00, 0x02]));
    assert!(val_neg.negative());
    assert_eq!(val_neg.magnitude(), &[0x01, 0x00, 0x02]);
}

#[test]
fn borrowed_dynamic_integer_case_11_significant_trailing_zero() {
    let val_pos = DynamicIntegerValue::from_parts(false, Cow::Borrowed(&[0x01, 0x00]));
    assert!(!val_pos.negative());
    assert_eq!(val_pos.magnitude(), &[0x01, 0x00]);

    let val_neg = DynamicIntegerValue::from_parts(true, Cow::Borrowed(&[0x01, 0x00]));
    assert!(val_neg.negative());
    assert_eq!(val_neg.magnitude(), &[0x01, 0x00]);
}

#[test]
fn borrowed_dynamic_integer_from_cow_owned() {
    let val = DynamicIntegerValue::from_parts(false, Cow::Owned(vec![0x00, 0x00, 0x05, 0x06]));
    assert!(!val.negative());
    assert_eq!(val.magnitude(), &[0x05, 0x06]);

    let zero = DynamicIntegerValue::from_parts(true, Cow::Owned(vec![0x00, 0x00]));
    assert!(!zero.negative());
    assert_eq!(zero.magnitude(), &[]);
}

#[test]
fn owned_dynamic_integer_case_1_zero_normal() {
    let val = OwnedDynamicInteger::from_parts(false, Box::new([]));
    assert!(!val.negative());
    assert_eq!(val.magnitude(), &[]);
}

#[test]
fn owned_dynamic_integer_case_2_negative_zero() {
    let val = OwnedDynamicInteger::from_parts(true, Box::new([]));
    assert!(!val.negative());
    assert_eq!(val.magnitude(), &[]);
}

#[test]
fn owned_dynamic_integer_case_3_zero_single_zero_byte() {
    let val_pos = OwnedDynamicInteger::from_parts(false, Box::new([0x00]));
    assert!(!val_pos.negative());
    assert_eq!(val_pos.magnitude(), &[]);

    let val_neg = OwnedDynamicInteger::from_parts(true, Box::new([0x00]));
    assert!(!val_neg.negative());
    assert_eq!(val_neg.magnitude(), &[]);
}

#[test]
fn owned_dynamic_integer_case_4_zero_multiple_zero_bytes() {
    let val_pos = OwnedDynamicInteger::from_parts(false, Box::new([0x00, 0x00, 0x00]));
    assert!(!val_pos.negative());
    assert_eq!(val_pos.magnitude(), &[]);

    let val_neg = OwnedDynamicInteger::from_parts(true, Box::new([0x00, 0x00, 0x00]));
    assert!(!val_neg.negative());
    assert_eq!(val_neg.magnitude(), &[]);
}

#[test]
fn owned_dynamic_integer_case_5_positive_one() {
    let val = OwnedDynamicInteger::from_parts(false, Box::new([0x01]));
    assert!(!val.negative());
    assert_eq!(val.magnitude(), &[0x01]);
}

#[test]
fn owned_dynamic_integer_case_6_negative_one() {
    let val = OwnedDynamicInteger::from_parts(true, Box::new([0x01]));
    assert!(val.negative());
    assert_eq!(val.magnitude(), &[0x01]);
}

#[test]
fn owned_dynamic_integer_case_7_magnitude_without_redundant_zeros() {
    let val_pos = OwnedDynamicInteger::from_parts(false, Box::new([0x01, 0x02]));
    assert!(!val_pos.negative());
    assert_eq!(val_pos.magnitude(), &[0x01, 0x02]);

    let val_neg = OwnedDynamicInteger::from_parts(true, Box::new([0x01, 0x02]));
    assert!(val_neg.negative());
    assert_eq!(val_neg.magnitude(), &[0x01, 0x02]);
}

#[test]
fn owned_dynamic_integer_case_8_single_leading_zero() {
    let val_pos = OwnedDynamicInteger::from_parts(false, Box::new([0x00, 0x01]));
    assert!(!val_pos.negative());
    assert_eq!(val_pos.magnitude(), &[0x01]);

    let val_neg = OwnedDynamicInteger::from_parts(true, Box::new([0x00, 0x01]));
    assert!(val_neg.negative());
    assert_eq!(val_neg.magnitude(), &[0x01]);
}

#[test]
fn owned_dynamic_integer_case_9_multiple_leading_zeros() {
    let val_pos = OwnedDynamicInteger::from_parts(false, Box::new([0x00, 0x00, 0x01]));
    assert!(!val_pos.negative());
    assert_eq!(val_pos.magnitude(), &[0x01]);

    let val_neg = OwnedDynamicInteger::from_parts(true, Box::new([0x00, 0x00, 0x01]));
    assert!(val_neg.negative());
    assert_eq!(val_neg.magnitude(), &[0x01]);
}

#[test]
fn owned_dynamic_integer_case_10_significant_internal_zero() {
    let val_pos = OwnedDynamicInteger::from_parts(false, Box::new([0x01, 0x00, 0x02]));
    assert!(!val_pos.negative());
    assert_eq!(val_pos.magnitude(), &[0x01, 0x00, 0x02]);

    let val_neg = OwnedDynamicInteger::from_parts(true, Box::new([0x01, 0x00, 0x02]));
    assert!(val_neg.negative());
    assert_eq!(val_neg.magnitude(), &[0x01, 0x00, 0x02]);
}

#[test]
fn owned_dynamic_integer_case_11_significant_trailing_zero() {
    let val_pos = OwnedDynamicInteger::from_parts(false, Box::new([0x01, 0x00]));
    assert!(!val_pos.negative());
    assert_eq!(val_pos.magnitude(), &[0x01, 0x00]);

    let val_neg = OwnedDynamicInteger::from_parts(true, Box::new([0x01, 0x00]));
    assert!(val_neg.negative());
    assert_eq!(val_neg.magnitude(), &[0x01, 0x00]);
}

#[test]
fn dynamic_value_with_canonical_integer() {
    let dyn_borrowed = DynamicValue::Integer(DynamicIntegerValue::from_parts(
        false,
        Cow::Borrowed(&[0x00, 0x0A]),
    ));
    match dyn_borrowed {
        DynamicValue::Integer(val) => {
            assert!(!val.negative());
            assert_eq!(val.magnitude(), &[0x0A]);
        }
        _ => panic!("expected DynamicValue::Integer"),
    }

    let dyn_owned = OwnedDynamicValue::Integer(OwnedDynamicInteger::from_parts(
        true,
        Box::new([0x00, 0x00, 0x0B]),
    ));
    match dyn_owned {
        OwnedDynamicValue::Integer(val) => {
            assert!(val.negative());
            assert_eq!(val.magnitude(), &[0x0B]);
        }
        _ => panic!("expected OwnedDynamicValue::Integer"),
    }
}
