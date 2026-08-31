extern crate alloc;

use alloc::borrow::Cow;
use alloc::boxed::Box;
use evo_values::definitions::value::{DynamicIntegerValue, DynamicValue, EnumPayload, Value};

#[test]
fn boolean_variants() {
    assert_eq!(Value::Boolean(true), Value::Boolean(true));
    assert_ne!(Value::Boolean(true), Value::Boolean(false));
}

#[test]
fn signed_integer_variants() {
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
fn unsigned_integer_variants() {
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
fn float_variants() {
    assert_eq!(Value::Float32(3.14), Value::Float32(3.14));
    assert_ne!(Value::Float32(3.14), Value::Float32(2.71));

    assert_eq!(
        Value::Float64(3.141592653589793),
        Value::Float64(3.141592653589793)
    );
    assert_ne!(
        Value::Float64(3.141592653589793),
        Value::Float64(2.718281828459045)
    );
}

#[test]
fn string_variant() {
    let text = "hola mundo";
    assert_eq!(Value::String(text), Value::String("hola mundo"));
    assert_ne!(Value::String(text), Value::String("otro"));
}

#[test]
fn dynamic_variants() {
    let dyn_int_pos = Value::Dynamic(DynamicValue::Integer(DynamicIntegerValue {
        negative: false,
        magnitude: Cow::Borrowed(&[0x01, 0x00]),
    }));
    let dyn_int_neg = Value::Dynamic(DynamicValue::Integer(DynamicIntegerValue {
        negative: true,
        magnitude: Cow::Borrowed(&[0x01, 0x00]),
    }));
    let dyn_int_zero = Value::Dynamic(DynamicValue::Integer(DynamicIntegerValue {
        negative: false,
        magnitude: Cow::Borrowed(&[]),
    }));

    assert_eq!(dyn_int_pos, dyn_int_pos);
    assert_ne!(dyn_int_pos, dyn_int_neg);
    assert_ne!(dyn_int_pos, dyn_int_zero);

    let dyn_f32 = Value::Dynamic(DynamicValue::Float32(1.5));
    let dyn_f64 = Value::Dynamic(DynamicValue::Float64(2.5));
    assert_eq!(dyn_f32, Value::Dynamic(DynamicValue::Float32(1.5)));
    assert_ne!(dyn_f32, Value::Dynamic(DynamicValue::Float32(3.5)));
    assert_eq!(dyn_f64, Value::Dynamic(DynamicValue::Float64(2.5)));
}

#[test]
fn struct_variant_preserves_order() {
    let s1 = Value::Struct(Box::new([
        Value::Int32(10),
        Value::String("test"),
        Value::Boolean(true),
    ]));
    let s2 = Value::Struct(Box::new([
        Value::Int32(10),
        Value::String("test"),
        Value::Boolean(true),
    ]));
    let s_diff_order = Value::Struct(Box::new([
        Value::String("test"),
        Value::Int32(10),
        Value::Boolean(true),
    ]));

    assert_eq!(s1, s2);
    assert_ne!(s1, s_diff_order);
}

#[test]
fn enum_variants() {
    let simple_enum = Value::Enum {
        variant: 0,
        payload: EnumPayload::Simple,
    };
    assert_eq!(
        simple_enum,
        Value::Enum {
            variant: 0,
            payload: EnumPayload::Simple,
        }
    );
    assert_ne!(
        simple_enum,
        Value::Enum {
            variant: 1,
            payload: EnumPayload::Simple,
        }
    );

    let assoc_enum = Value::Enum {
        variant: 1,
        payload: EnumPayload::Associated(Box::new(Value::String("error message"))),
    };
    assert_eq!(
        assoc_enum,
        Value::Enum {
            variant: 1,
            payload: EnumPayload::Associated(Box::new(Value::String("error message"))),
        }
    );

    let struct_enum = Value::Enum {
        variant: 2,
        payload: EnumPayload::Structured {
            fields: Box::new([Value::Int32(1), Value::Int32(2)]),
        },
    };
    assert_eq!(
        struct_enum,
        Value::Enum {
            variant: 2,
            payload: EnumPayload::Structured {
                fields: Box::new([Value::Int32(1), Value::Int32(2)]),
            },
        }
    );
}
