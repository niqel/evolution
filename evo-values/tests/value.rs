extern crate alloc;

use alloc::borrow::Cow;
use alloc::boxed::Box;
use evo_values::definitions::value::{
    DynamicIntegerValue, DynamicValue, EnumPayload, OwnedDynamicInteger, OwnedDynamicValue,
    OwnedEnumPayload, OwnedValue, Value,
};

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
    assert_eq!(OwnedValue::Float32(3.14), OwnedValue::Float32(3.14));
    assert_ne!(OwnedValue::Float32(3.14), OwnedValue::Float32(2.71));

    assert_eq!(
        OwnedValue::Float64(3.141592653589793),
        OwnedValue::Float64(3.141592653589793)
    );
    assert_ne!(
        OwnedValue::Float64(3.141592653589793),
        OwnedValue::Float64(2.718281828459045)
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

#[test]
fn owned_dynamic_variants() {
    let dyn_int_pos = OwnedValue::Dynamic(OwnedDynamicValue::Integer(OwnedDynamicInteger {
        negative: false,
        magnitude: Box::new([0x01, 0x00]),
    }));
    let dyn_int_neg = OwnedValue::Dynamic(OwnedDynamicValue::Integer(OwnedDynamicInteger {
        negative: true,
        magnitude: Box::new([0x01, 0x00]),
    }));
    let dyn_int_zero = OwnedValue::Dynamic(OwnedDynamicValue::Integer(OwnedDynamicInteger {
        negative: false,
        magnitude: Box::new([]),
    }));

    assert_eq!(dyn_int_pos, dyn_int_pos);
    assert_ne!(dyn_int_pos, dyn_int_neg);
    assert_ne!(dyn_int_pos, dyn_int_zero);

    let dyn_f32 = OwnedValue::Dynamic(OwnedDynamicValue::Float32(1.5));
    let dyn_f64 = OwnedValue::Dynamic(OwnedDynamicValue::Float64(2.5));
    assert_eq!(
        dyn_f32,
        OwnedValue::Dynamic(OwnedDynamicValue::Float32(1.5))
    );
    assert_ne!(
        dyn_f32,
        OwnedValue::Dynamic(OwnedDynamicValue::Float32(3.5))
    );
    assert_eq!(
        dyn_f64,
        OwnedValue::Dynamic(OwnedDynamicValue::Float64(2.5))
    );
}

#[test]
fn owned_struct_variant_preserves_order() {
    let s1 = OwnedValue::Struct(Box::new([
        OwnedValue::Int32(10),
        OwnedValue::String(Box::from("test")),
        OwnedValue::Boolean(true),
    ]));
    let s2 = OwnedValue::Struct(Box::new([
        OwnedValue::Int32(10),
        OwnedValue::String(Box::from("test")),
        OwnedValue::Boolean(true),
    ]));
    let s_diff_order = OwnedValue::Struct(Box::new([
        OwnedValue::String(Box::from("test")),
        OwnedValue::Int32(10),
        OwnedValue::Boolean(true),
    ]));

    assert_eq!(s1, s2);
    assert_ne!(s1, s_diff_order);
}

#[test]
fn owned_enum_variants() {
    let simple_enum = OwnedValue::Enum {
        variant: 0,
        payload: OwnedEnumPayload::Simple,
    };
    assert_eq!(
        simple_enum,
        OwnedValue::Enum {
            variant: 0,
            payload: OwnedEnumPayload::Simple,
        }
    );
    assert_ne!(
        simple_enum,
        OwnedValue::Enum {
            variant: 1,
            payload: OwnedEnumPayload::Simple,
        }
    );

    let assoc_enum = OwnedValue::Enum {
        variant: 1,
        payload: OwnedEnumPayload::Associated(Box::new(OwnedValue::String(Box::from(
            "error message",
        )))),
    };
    assert_eq!(
        assoc_enum,
        OwnedValue::Enum {
            variant: 1,
            payload: OwnedEnumPayload::Associated(Box::new(OwnedValue::String(Box::from(
                "error message"
            )))),
        }
    );

    let struct_enum = OwnedValue::Enum {
        variant: 2,
        payload: OwnedEnumPayload::Structured {
            fields: Box::new([OwnedValue::Int32(1), OwnedValue::Int32(2)]),
        },
    };
    assert_eq!(
        struct_enum,
        OwnedValue::Enum {
            variant: 2,
            payload: OwnedEnumPayload::Structured {
                fields: Box::new([OwnedValue::Int32(1), OwnedValue::Int32(2)]),
            },
        }
    );
}
