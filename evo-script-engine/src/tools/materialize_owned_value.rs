use alloc::boxed::Box;
use alloc::vec::Vec;
use num_bigint::{BigInt, Sign};

use crate::data::compiled::identities::VariantDiscriminant;
use crate::data::vm::backing::{
    DynamicIntegerBacking, EnumBacking, ExecutionBackingStore, RuntimeEnumPayload, StructBacking,
};
use crate::data::vm::values::{
    DynamicIntegerBackingId, DynamicIntegerBackingRef, DynamicValue as RuntimeDynamicValue,
    EnumBackingId, RuntimeValue, StringBackingId, StringBackingRef, StructBackingId,
};
use evo_values::{OwnedDynamicValue, OwnedEnumPayload, OwnedValue};

pub type MaterializeOwnedValue = fn(OwnedValue, &mut ExecutionBackingStore) -> RuntimeValue;

pub fn materialize_owned_value(
    value: OwnedValue,
    backing_store: &mut ExecutionBackingStore,
) -> RuntimeValue {
    match value {
        OwnedValue::Boolean(b) => RuntimeValue::Boolean(b),

        OwnedValue::Int8(v) => RuntimeValue::Int8(v),
        OwnedValue::Int16(v) => RuntimeValue::Int16(v),
        OwnedValue::Int32(v) => RuntimeValue::Int32(v),
        OwnedValue::Int64(v) => RuntimeValue::Int64(v),
        OwnedValue::Int128(v) => RuntimeValue::Int128(v),

        OwnedValue::Uint8(v) => RuntimeValue::Uint8(v),
        OwnedValue::Uint16(v) => RuntimeValue::Uint16(v),
        OwnedValue::Uint32(v) => RuntimeValue::Uint32(v),
        OwnedValue::Uint64(v) => RuntimeValue::Uint64(v),
        OwnedValue::Uint128(v) => RuntimeValue::Uint128(v),

        OwnedValue::Float32(v) => RuntimeValue::Float32(v),
        OwnedValue::Float64(v) => RuntimeValue::Float64(v),

        OwnedValue::String(s) => {
            let id = StringBackingId(backing_store.strings.len());
            backing_store.strings.push(s);
            RuntimeValue::String(StringBackingRef::Execution(id))
        }

        OwnedValue::Dynamic(dyn_val) => match dyn_val {
            OwnedDynamicValue::Integer(dyn_int) => {
                let sign = if dyn_int.negative {
                    Sign::Minus
                } else {
                    Sign::Plus
                };
                let big_int = BigInt::from_bytes_be(sign, &dyn_int.magnitude);
                let id = DynamicIntegerBackingId(backing_store.dynamic_integers.len());
                backing_store
                    .dynamic_integers
                    .push(DynamicIntegerBacking { value: big_int });
                RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                    DynamicIntegerBackingRef::Execution(id),
                ))
            }
            OwnedDynamicValue::Float32(v) => RuntimeValue::Dynamic(RuntimeDynamicValue::Float32(v)),
            OwnedDynamicValue::Float64(v) => RuntimeValue::Dynamic(RuntimeDynamicValue::Float64(v)),
        },

        OwnedValue::Struct(fields) => {
            let mut runtime_fields = Vec::with_capacity(fields.len());
            for f in fields.into_vec() {
                runtime_fields.push(materialize_owned_value(f, backing_store));
            }
            let id = StructBackingId(backing_store.structs.len());
            backing_store.structs.push(StructBacking {
                fields: runtime_fields.into_boxed_slice(),
            });
            RuntimeValue::Struct(id)
        }

        OwnedValue::Enum { variant, payload } => {
            let runtime_payload = match payload {
                OwnedEnumPayload::Simple => RuntimeEnumPayload::Simple,
                OwnedEnumPayload::Associated(associated_val) => {
                    let runtime_val = materialize_owned_value(*associated_val, backing_store);
                    RuntimeEnumPayload::Associated(runtime_val)
                }
                OwnedEnumPayload::Structured { fields } => {
                    let mut runtime_fields = Vec::with_capacity(fields.len());
                    for f in fields.into_vec() {
                        runtime_fields.push(materialize_owned_value(f, backing_store));
                    }
                    RuntimeEnumPayload::Structured {
                        fields: runtime_fields.into_boxed_slice(),
                    }
                }
            };
            let id = EnumBackingId(backing_store.enums.len());
            backing_store.enums.push(EnumBacking {
                variant: VariantDiscriminant(variant),
                payload: runtime_payload,
            });
            RuntimeValue::Enum(id)
        }
    }
}

pub const MATERIALIZE_OWNED_VALUE: MaterializeOwnedValue = materialize_owned_value;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use alloc::vec;
    use evo_values::OwnedDynamicInteger;

    fn empty_store() -> ExecutionBackingStore {
        ExecutionBackingStore {
            strings: Vec::new(),
            dynamic_integers: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
        }
    }

    #[test]
    fn typed_binding() {
        let implementation: MaterializeOwnedValue = materialize_owned_value;
        let binding: MaterializeOwnedValue = MATERIALIZE_OWNED_VALUE;
        assert_eq!(implementation as usize, binding as usize);
    }

    #[test]
    fn fixed_scalars_inline_no_backing() {
        let mut store = empty_store();

        match materialize_owned_value(OwnedValue::Boolean(true), &mut store) {
            RuntimeValue::Boolean(b) => assert!(b),
            _ => panic!("expected Boolean"),
        }
        match materialize_owned_value(OwnedValue::Int8(-8), &mut store) {
            RuntimeValue::Int8(v) => assert_eq!(v, -8),
            _ => panic!("expected Int8"),
        }
        match materialize_owned_value(OwnedValue::Int16(-16), &mut store) {
            RuntimeValue::Int16(v) => assert_eq!(v, -16),
            _ => panic!("expected Int16"),
        }
        match materialize_owned_value(OwnedValue::Int32(-32), &mut store) {
            RuntimeValue::Int32(v) => assert_eq!(v, -32),
            _ => panic!("expected Int32"),
        }
        match materialize_owned_value(OwnedValue::Int64(-64), &mut store) {
            RuntimeValue::Int64(v) => assert_eq!(v, -64),
            _ => panic!("expected Int64"),
        }
        match materialize_owned_value(OwnedValue::Int128(-128), &mut store) {
            RuntimeValue::Int128(v) => assert_eq!(v, -128),
            _ => panic!("expected Int128"),
        }
        match materialize_owned_value(OwnedValue::Uint8(8), &mut store) {
            RuntimeValue::Uint8(v) => assert_eq!(v, 8),
            _ => panic!("expected Uint8"),
        }
        match materialize_owned_value(OwnedValue::Uint16(16), &mut store) {
            RuntimeValue::Uint16(v) => assert_eq!(v, 16),
            _ => panic!("expected Uint16"),
        }
        match materialize_owned_value(OwnedValue::Uint32(32), &mut store) {
            RuntimeValue::Uint32(v) => assert_eq!(v, 32),
            _ => panic!("expected Uint32"),
        }
        match materialize_owned_value(OwnedValue::Uint64(64), &mut store) {
            RuntimeValue::Uint64(v) => assert_eq!(v, 64),
            _ => panic!("expected Uint64"),
        }
        match materialize_owned_value(OwnedValue::Uint128(128), &mut store) {
            RuntimeValue::Uint128(v) => assert_eq!(v, 128),
            _ => panic!("expected Uint128"),
        }
        match materialize_owned_value(OwnedValue::Float32(1.5), &mut store) {
            RuntimeValue::Float32(v) => assert_eq!(v, 1.5),
            _ => panic!("expected Float32"),
        }
        match materialize_owned_value(OwnedValue::Float64(2.5), &mut store) {
            RuntimeValue::Float64(v) => assert_eq!(v, 2.5),
            _ => panic!("expected Float64"),
        }

        // Confirm backing stores remain empty
        assert!(store.strings.is_empty());
        assert!(store.dynamic_integers.is_empty());
        assert!(store.structs.is_empty());
        assert!(store.enums.is_empty());
    }

    #[test]
    fn string_ownership_transfer() {
        let mut store = empty_store();
        let owned_str = "hello owned".to_string().into_boxed_str();
        let original_ptr = owned_str.as_ptr();

        let runtime_val = materialize_owned_value(OwnedValue::String(owned_str), &mut store);

        match runtime_val {
            RuntimeValue::String(StringBackingRef::Execution(id)) => {
                assert_eq!(id.0, 0);
            }
            _ => panic!("expected Execution string"),
        }
        assert_eq!(store.strings.len(), 1);
        assert_eq!(&*store.strings[0], "hello owned");
        assert_eq!(store.strings[0].as_ptr(), original_ptr);
    }

    #[test]
    fn existing_string_backing() {
        let mut store = empty_store();
        store.strings.push("existing".to_string().into_boxed_str());

        let owned_str = "second".to_string().into_boxed_str();
        let original_ptr = owned_str.as_ptr();

        let runtime_val = materialize_owned_value(OwnedValue::String(owned_str), &mut store);

        match runtime_val {
            RuntimeValue::String(StringBackingRef::Execution(id)) => {
                assert_eq!(id.0, 1);
            }
            _ => panic!("expected Execution string"),
        }
        assert_eq!(store.strings.len(), 2);
        assert_eq!(&*store.strings[0], "existing");
        assert_eq!(&*store.strings[1], "second");
        assert_eq!(store.strings[1].as_ptr(), original_ptr);
    }

    #[test]
    fn dynamic_floats_inline_no_backing() {
        let mut store = empty_store();

        let val_f32 = materialize_owned_value(
            OwnedValue::Dynamic(OwnedDynamicValue::Float32(3.5)),
            &mut store,
        );
        match val_f32 {
            RuntimeValue::Dynamic(RuntimeDynamicValue::Float32(v)) => assert_eq!(v, 3.5),
            _ => panic!("expected Float32 dynamic"),
        }

        let val_f64 = materialize_owned_value(
            OwnedValue::Dynamic(OwnedDynamicValue::Float64(7.5)),
            &mut store,
        );
        match val_f64 {
            RuntimeValue::Dynamic(RuntimeDynamicValue::Float64(v)) => assert_eq!(v, 7.5),
            _ => panic!("expected Float64 dynamic"),
        }

        assert!(store.dynamic_integers.is_empty());
    }

    #[test]
    fn dynamic_integer_positive() {
        let mut store = empty_store();
        let val = OwnedValue::Dynamic(OwnedDynamicValue::Integer(OwnedDynamicInteger {
            negative: false,
            magnitude: vec![42].into_boxed_slice(),
        }));

        let runtime_val = materialize_owned_value(val, &mut store);
        match runtime_val {
            RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                DynamicIntegerBackingRef::Execution(id),
            )) => {
                assert_eq!(id.0, 0);
            }
            _ => panic!("expected dynamic integer"),
        }
        assert_eq!(store.dynamic_integers.len(), 1);
        assert_eq!(store.dynamic_integers[0].value, BigInt::from(42));
    }

    #[test]
    fn dynamic_integer_negative() {
        let mut store = empty_store();
        let val = OwnedValue::Dynamic(OwnedDynamicValue::Integer(OwnedDynamicInteger {
            negative: true,
            magnitude: vec![42].into_boxed_slice(),
        }));

        let runtime_val = materialize_owned_value(val, &mut store);
        match runtime_val {
            RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                DynamicIntegerBackingRef::Execution(id),
            )) => {
                assert_eq!(id.0, 0);
            }
            _ => panic!("expected dynamic integer"),
        }
        assert_eq!(store.dynamic_integers.len(), 1);
        assert_eq!(store.dynamic_integers[0].value, BigInt::from(-42));
    }

    #[test]
    fn dynamic_integer_zero() {
        let mut store = empty_store();

        // 1. negative = false, magnitude = []
        let val1 = OwnedValue::Dynamic(OwnedDynamicValue::Integer(OwnedDynamicInteger {
            negative: false,
            magnitude: vec![].into_boxed_slice(),
        }));
        let rt1 = materialize_owned_value(val1, &mut store);
        match rt1 {
            RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                DynamicIntegerBackingRef::Execution(id),
            )) => {
                assert_eq!(id.0, 0);
            }
            _ => panic!("expected dynamic integer"),
        }
        assert_eq!(store.dynamic_integers[0].value, BigInt::from(0));

        // 2. negative = true, magnitude = []
        let val2 = OwnedValue::Dynamic(OwnedDynamicValue::Integer(OwnedDynamicInteger {
            negative: true,
            magnitude: vec![].into_boxed_slice(),
        }));
        let rt2 = materialize_owned_value(val2, &mut store);
        match rt2 {
            RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                DynamicIntegerBackingRef::Execution(id),
            )) => {
                assert_eq!(id.0, 1);
            }
            _ => panic!("expected dynamic integer"),
        }
        assert_eq!(store.dynamic_integers[1].value, BigInt::from(0));
    }

    #[test]
    fn dynamic_integer_greater_than_u128() {
        let mut store = empty_store();
        // 2^128 = 340282366920938463463374607431768211456
        let mut mag = vec![0u8; 17];
        mag[0] = 1;
        let val = OwnedValue::Dynamic(OwnedDynamicValue::Integer(OwnedDynamicInteger {
            negative: false,
            magnitude: mag.into_boxed_slice(),
        }));

        let runtime_val = materialize_owned_value(val, &mut store);
        match runtime_val {
            RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                DynamicIntegerBackingRef::Execution(id),
            )) => {
                assert_eq!(id.0, 0);
            }
            _ => panic!("expected dynamic integer"),
        }
        let expected_bigint = BigInt::parse_bytes(b"340282366920938463463374607431768211456", 10)
            .expect("valid decimal");
        assert_eq!(store.dynamic_integers[0].value, expected_bigint);
    }

    #[test]
    fn existing_dynamic_backing() {
        let mut store = empty_store();
        store.dynamic_integers.push(DynamicIntegerBacking {
            value: BigInt::from(100),
        });

        let val = OwnedValue::Dynamic(OwnedDynamicValue::Integer(OwnedDynamicInteger {
            negative: false,
            magnitude: vec![200].into_boxed_slice(),
        }));

        let rt = materialize_owned_value(val, &mut store);
        match rt {
            RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                DynamicIntegerBackingRef::Execution(id),
            )) => {
                assert_eq!(id.0, 1);
            }
            _ => panic!("expected dynamic integer"),
        }
        assert_eq!(store.dynamic_integers.len(), 2);
        assert_eq!(store.dynamic_integers[0].value, BigInt::from(100));
        assert_eq!(store.dynamic_integers[1].value, BigInt::from(200));
    }

    #[test]
    fn struct_materialization_and_field_order() {
        let mut store = empty_store();
        let owned_struct = OwnedValue::Struct(
            vec![
                OwnedValue::Int32(100),
                OwnedValue::String("field str".to_string().into_boxed_str()),
                OwnedValue::Dynamic(OwnedDynamicValue::Integer(OwnedDynamicInteger {
                    negative: false,
                    magnitude: vec![50].into_boxed_slice(),
                })),
            ]
            .into_boxed_slice(),
        );

        let rt = materialize_owned_value(owned_struct, &mut store);
        match rt {
            RuntimeValue::Struct(id) => assert_eq!(id.0, 0),
            _ => panic!("expected Struct"),
        }

        assert_eq!(store.strings.len(), 1);
        assert_eq!(&*store.strings[0], "field str");

        assert_eq!(store.dynamic_integers.len(), 1);
        assert_eq!(store.dynamic_integers[0].value, BigInt::from(50));

        assert_eq!(store.structs.len(), 1);
        assert_eq!(store.structs[0].fields.len(), 3);
        match store.structs[0].fields[0] {
            RuntimeValue::Int32(v) => assert_eq!(v, 100),
            _ => panic!("expected Int32"),
        }
        match store.structs[0].fields[1] {
            RuntimeValue::String(StringBackingRef::Execution(id)) => assert_eq!(id.0, 0),
            _ => panic!("expected Execution string"),
        }
        match store.structs[0].fields[2] {
            RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                DynamicIntegerBackingRef::Execution(id),
            )) => assert_eq!(id.0, 0),
            _ => panic!("expected dynamic integer"),
        }
    }

    #[test]
    fn empty_struct() {
        let mut store = empty_store();
        let empty_struct = OwnedValue::Struct(vec![].into_boxed_slice());

        let rt = materialize_owned_value(empty_struct, &mut store);
        match rt {
            RuntimeValue::Struct(id) => assert_eq!(id.0, 0),
            _ => panic!("expected Struct"),
        }
        assert_eq!(store.structs.len(), 1);
        assert_eq!(store.structs[0].fields.len(), 0);
    }

    #[test]
    fn nested_struct_child_before_parent() {
        let mut store = empty_store();
        let inner = OwnedValue::Struct(
            vec![OwnedValue::String("inner".to_string().into_boxed_str())].into_boxed_slice(),
        );
        let outer = OwnedValue::Struct(vec![OwnedValue::Int32(1), inner].into_boxed_slice());

        let rt = materialize_owned_value(outer, &mut store);

        // String backing at 0
        assert_eq!(store.strings.len(), 1);
        assert_eq!(&*store.strings[0], "inner");

        // Inner struct at 0, outer struct at 1
        assert_eq!(store.structs.len(), 2);
        match rt {
            RuntimeValue::Struct(id) => assert_eq!(id.0, 1),
            _ => panic!("expected outer Struct at 1"),
        }

        // Inner struct fields
        assert_eq!(store.structs[0].fields.len(), 1);
        match store.structs[0].fields[0] {
            RuntimeValue::String(StringBackingRef::Execution(id)) => assert_eq!(id.0, 0),
            _ => panic!("expected Execution string"),
        }

        // Outer struct fields
        assert_eq!(store.structs[1].fields.len(), 2);
        match store.structs[1].fields[0] {
            RuntimeValue::Int32(v) => assert_eq!(v, 1),
            _ => panic!("expected Int32"),
        }
        match store.structs[1].fields[1] {
            RuntimeValue::Struct(id) => assert_eq!(id.0, 0),
            _ => panic!("expected Struct at 0"),
        }
    }

    #[test]
    fn existing_struct_backing() {
        let mut store = empty_store();
        store.structs.push(StructBacking {
            fields: vec![RuntimeValue::Int32(999)].into_boxed_slice(),
        });

        let new_struct = OwnedValue::Struct(vec![OwnedValue::Int32(888)].into_boxed_slice());
        let rt = materialize_owned_value(new_struct, &mut store);

        match rt {
            RuntimeValue::Struct(id) => assert_eq!(id.0, 1),
            _ => panic!("expected Struct at 1"),
        }
        assert_eq!(store.structs.len(), 2);
        match store.structs[0].fields[0] {
            RuntimeValue::Int32(v) => assert_eq!(v, 999),
            _ => panic!("expected Int32"),
        }
        match store.structs[1].fields[0] {
            RuntimeValue::Int32(v) => assert_eq!(v, 888),
            _ => panic!("expected Int32"),
        }
    }

    #[test]
    fn enum_simple() {
        let mut store = empty_store();
        let val = OwnedValue::Enum {
            variant: 7,
            payload: OwnedEnumPayload::Simple,
        };

        let rt = materialize_owned_value(val, &mut store);
        match rt {
            RuntimeValue::Enum(id) => assert_eq!(id.0, 0),
            _ => panic!("expected Enum at 0"),
        }

        assert_eq!(store.enums.len(), 1);
        assert_eq!(store.enums[0].variant.0, 7);
        assert!(matches!(store.enums[0].payload, RuntimeEnumPayload::Simple));
    }

    #[test]
    fn enum_associated_child_before_parent() {
        let mut store = empty_store();
        let val = OwnedValue::Enum {
            variant: 3,
            payload: OwnedEnumPayload::Associated(Box::new(OwnedValue::String(
                "associated str".to_string().into_boxed_str(),
            ))),
        };

        let rt = materialize_owned_value(val, &mut store);
        match rt {
            RuntimeValue::Enum(id) => assert_eq!(id.0, 0),
            _ => panic!("expected Enum at 0"),
        }

        // String child backing at 0
        assert_eq!(store.strings.len(), 1);
        assert_eq!(&*store.strings[0], "associated str");

        assert_eq!(store.enums.len(), 1);
        assert_eq!(store.enums[0].variant.0, 3);
        match &store.enums[0].payload {
            RuntimeEnumPayload::Associated(RuntimeValue::String(StringBackingRef::Execution(
                id,
            ))) => {
                assert_eq!(id.0, 0);
            }
            _ => panic!("expected Associated with Execution string"),
        }
    }

    #[test]
    fn enum_structured_cardinality_and_order() {
        let mut store = empty_store();
        // 1. Non-empty structured
        let val_non_empty = OwnedValue::Enum {
            variant: 1,
            payload: OwnedEnumPayload::Structured {
                fields: vec![
                    OwnedValue::Int32(10),
                    OwnedValue::String("field1".to_string().into_boxed_str()),
                ]
                .into_boxed_slice(),
            },
        };

        let rt1 = materialize_owned_value(val_non_empty, &mut store);
        match rt1 {
            RuntimeValue::Enum(id) => assert_eq!(id.0, 0),
            _ => panic!("expected Enum at 0"),
        }

        assert_eq!(store.strings.len(), 1);
        assert_eq!(&*store.strings[0], "field1");

        assert_eq!(store.enums.len(), 1);
        assert_eq!(store.enums[0].variant.0, 1);
        match &store.enums[0].payload {
            RuntimeEnumPayload::Structured { fields } => {
                assert_eq!(fields.len(), 2);
                match fields[0] {
                    RuntimeValue::Int32(v) => assert_eq!(v, 10),
                    _ => panic!("expected Int32"),
                }
                match fields[1] {
                    RuntimeValue::String(StringBackingRef::Execution(id)) => assert_eq!(id.0, 0),
                    _ => panic!("expected Execution string"),
                }
            }
            _ => panic!("expected structured payload"),
        }

        // 2. Empty structured
        let val_empty = OwnedValue::Enum {
            variant: 2,
            payload: OwnedEnumPayload::Structured {
                fields: vec![].into_boxed_slice(),
            },
        };
        let rt2 = materialize_owned_value(val_empty, &mut store);
        match rt2 {
            RuntimeValue::Enum(id) => assert_eq!(id.0, 1),
            _ => panic!("expected Enum at 1"),
        }
        assert_eq!(store.enums.len(), 2);
        assert_eq!(store.enums[1].variant.0, 2);
        match &store.enums[1].payload {
            RuntimeEnumPayload::Structured { fields } => {
                assert_eq!(fields.len(), 0);
            }
            _ => panic!("expected structured payload"),
        }
    }

    #[test]
    fn existing_enum_backing() {
        let mut store = empty_store();
        store.enums.push(EnumBacking {
            variant: VariantDiscriminant(0),
            payload: RuntimeEnumPayload::Simple,
        });

        let val = OwnedValue::Enum {
            variant: 1,
            payload: OwnedEnumPayload::Simple,
        };
        let rt = materialize_owned_value(val, &mut store);

        match rt {
            RuntimeValue::Enum(id) => assert_eq!(id.0, 1),
            _ => panic!("expected Enum at 1"),
        }
        assert_eq!(store.enums.len(), 2);
        assert_eq!(store.enums[0].variant.0, 0);
        assert_eq!(store.enums[1].variant.0, 1);
    }

    #[test]
    fn deep_composite_tree() {
        let mut store = empty_store();
        let val = OwnedValue::Enum {
            variant: 0,
            payload: OwnedEnumPayload::Structured {
                fields: vec![
                    OwnedValue::Struct(
                        vec![
                            OwnedValue::String("composite leaf".to_string().into_boxed_str()),
                            OwnedValue::Dynamic(OwnedDynamicValue::Integer(OwnedDynamicInteger {
                                negative: false,
                                magnitude: vec![77].into_boxed_slice(),
                            })),
                        ]
                        .into_boxed_slice(),
                    ),
                    OwnedValue::Enum {
                        variant: 4,
                        payload: OwnedEnumPayload::Associated(Box::new(OwnedValue::String(
                            "associated leaf".to_string().into_boxed_str(),
                        ))),
                    },
                ]
                .into_boxed_slice(),
            },
        };

        let rt = materialize_owned_value(val, &mut store);

        // Verification:
        // 1. Strings: "composite leaf" at 0, "associated leaf" at 1
        assert_eq!(store.strings.len(), 2);
        assert_eq!(&*store.strings[0], "composite leaf");
        assert_eq!(&*store.strings[1], "associated leaf");

        // 2. Dynamic integer at 0
        assert_eq!(store.dynamic_integers.len(), 1);
        assert_eq!(store.dynamic_integers[0].value, BigInt::from(77));

        // 3. Struct at 0
        assert_eq!(store.structs.len(), 1);
        assert_eq!(store.structs[0].fields.len(), 2);
        match store.structs[0].fields[0] {
            RuntimeValue::String(StringBackingRef::Execution(id)) => assert_eq!(id.0, 0),
            _ => panic!("expected Execution string at 0"),
        }
        match store.structs[0].fields[1] {
            RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                DynamicIntegerBackingRef::Execution(id),
            )) => assert_eq!(id.0, 0),
            _ => panic!("expected Execution dynamic integer at 0"),
        }

        // 4. Enums: Associated enum at 0, Outer Structured enum at 1
        assert_eq!(store.enums.len(), 2);
        match rt {
            RuntimeValue::Enum(id) => assert_eq!(id.0, 1),
            _ => panic!("expected outer Enum at 1"),
        }

        // Child enum (Associated)
        assert_eq!(store.enums[0].variant.0, 4);
        match &store.enums[0].payload {
            RuntimeEnumPayload::Associated(RuntimeValue::String(StringBackingRef::Execution(
                id,
            ))) => {
                assert_eq!(id.0, 1);
            }
            _ => panic!("expected Execution string at 1"),
        }

        // Parent enum (Structured)
        assert_eq!(store.enums[1].variant.0, 0);
        match &store.enums[1].payload {
            RuntimeEnumPayload::Structured { fields } => {
                assert_eq!(fields.len(), 2);
                match fields[0] {
                    RuntimeValue::Struct(id) => assert_eq!(id.0, 0),
                    _ => panic!("expected Struct at 0"),
                }
                match fields[1] {
                    RuntimeValue::Enum(id) => assert_eq!(id.0, 0),
                    _ => panic!("expected Enum at 0"),
                }
            }
            _ => panic!("expected structured payload"),
        }
    }
}
