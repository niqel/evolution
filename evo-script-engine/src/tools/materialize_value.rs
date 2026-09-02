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
use evo_values::{DynamicValue as InterchangeDynamicValue, EnumPayload, Value};

pub type MaterializeValue =
    for<'value> fn(&Value<'value>, &mut ExecutionBackingStore) -> RuntimeValue;

pub fn materialize_value<'value>(
    value: &Value<'value>,
    backing_store: &mut ExecutionBackingStore,
) -> RuntimeValue {
    match value {
        Value::Boolean(b) => RuntimeValue::Boolean(*b),

        Value::Int8(v) => RuntimeValue::Int8(*v),
        Value::Int16(v) => RuntimeValue::Int16(*v),
        Value::Int32(v) => RuntimeValue::Int32(*v),
        Value::Int64(v) => RuntimeValue::Int64(*v),
        Value::Int128(v) => RuntimeValue::Int128(*v),

        Value::Uint8(v) => RuntimeValue::Uint8(*v),
        Value::Uint16(v) => RuntimeValue::Uint16(*v),
        Value::Uint32(v) => RuntimeValue::Uint32(*v),
        Value::Uint64(v) => RuntimeValue::Uint64(*v),
        Value::Uint128(v) => RuntimeValue::Uint128(*v),

        Value::Float32(v) => RuntimeValue::Float32(*v),
        Value::Float64(v) => RuntimeValue::Float64(*v),

        Value::String(s) => {
            let id = StringBackingId(backing_store.strings.len());
            let boxed_str: Box<str> = (*s).into();
            backing_store.strings.push(boxed_str);
            RuntimeValue::String(StringBackingRef::Execution(id))
        }

        Value::Dynamic(dyn_val) => match dyn_val {
            InterchangeDynamicValue::Integer(dyn_int) => {
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
            InterchangeDynamicValue::Float32(v) => {
                RuntimeValue::Dynamic(RuntimeDynamicValue::Float32(*v))
            }
            InterchangeDynamicValue::Float64(v) => {
                RuntimeValue::Dynamic(RuntimeDynamicValue::Float64(*v))
            }
        },

        Value::Struct(fields) => {
            let mut runtime_fields = Vec::with_capacity(fields.len());
            for f in fields.iter() {
                runtime_fields.push(materialize_value(f, backing_store));
            }
            let id = StructBackingId(backing_store.structs.len());
            backing_store.structs.push(StructBacking {
                fields: runtime_fields.into_boxed_slice(),
            });
            RuntimeValue::Struct(id)
        }

        Value::Enum { variant, payload } => {
            let runtime_payload = match payload {
                EnumPayload::Simple => RuntimeEnumPayload::Simple,
                EnumPayload::Associated(associated_val) => {
                    let runtime_val = materialize_value(associated_val, backing_store);
                    RuntimeEnumPayload::Associated(runtime_val)
                }
                EnumPayload::Structured { fields } => {
                    let mut runtime_fields = Vec::with_capacity(fields.len());
                    for f in fields.iter() {
                        runtime_fields.push(materialize_value(f, backing_store));
                    }
                    RuntimeEnumPayload::Structured {
                        fields: runtime_fields.into_boxed_slice(),
                    }
                }
            };
            let id = EnumBackingId(backing_store.enums.len());
            backing_store.enums.push(EnumBacking {
                variant: VariantDiscriminant(*variant),
                payload: runtime_payload,
            });
            RuntimeValue::Enum(id)
        }
    }
}

pub const MATERIALIZE_VALUE: MaterializeValue = materialize_value;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::borrow::Cow;
    use alloc::string::ToString;
    use alloc::vec;
    use evo_values::DynamicIntegerValue;

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
        let implementation: MaterializeValue = materialize_value;
        let binding: MaterializeValue = MATERIALIZE_VALUE;
        assert_eq!(implementation as usize, binding as usize);
    }

    #[test]
    fn fixed_scalars_inline_no_backing() {
        let mut store = empty_store();

        match materialize_value(&Value::Boolean(true), &mut store) {
            RuntimeValue::Boolean(b) => assert!(b),
            _ => panic!("expected Boolean"),
        }
        match materialize_value(&Value::Int8(-8), &mut store) {
            RuntimeValue::Int8(v) => assert_eq!(v, -8),
            _ => panic!("expected Int8"),
        }
        match materialize_value(&Value::Int16(-16), &mut store) {
            RuntimeValue::Int16(v) => assert_eq!(v, -16),
            _ => panic!("expected Int16"),
        }
        match materialize_value(&Value::Int32(-32), &mut store) {
            RuntimeValue::Int32(v) => assert_eq!(v, -32),
            _ => panic!("expected Int32"),
        }
        match materialize_value(&Value::Int64(-64), &mut store) {
            RuntimeValue::Int64(v) => assert_eq!(v, -64),
            _ => panic!("expected Int64"),
        }
        match materialize_value(&Value::Int128(-128), &mut store) {
            RuntimeValue::Int128(v) => assert_eq!(v, -128),
            _ => panic!("expected Int128"),
        }
        match materialize_value(&Value::Uint8(8), &mut store) {
            RuntimeValue::Uint8(v) => assert_eq!(v, 8),
            _ => panic!("expected Uint8"),
        }
        match materialize_value(&Value::Uint16(16), &mut store) {
            RuntimeValue::Uint16(v) => assert_eq!(v, 16),
            _ => panic!("expected Uint16"),
        }
        match materialize_value(&Value::Uint32(32), &mut store) {
            RuntimeValue::Uint32(v) => assert_eq!(v, 32),
            _ => panic!("expected Uint32"),
        }
        match materialize_value(&Value::Uint64(64), &mut store) {
            RuntimeValue::Uint64(v) => assert_eq!(v, 64),
            _ => panic!("expected Uint64"),
        }
        match materialize_value(&Value::Uint128(128), &mut store) {
            RuntimeValue::Uint128(v) => assert_eq!(v, 128),
            _ => panic!("expected Uint128"),
        }
        match materialize_value(&Value::Float32(3.5), &mut store) {
            RuntimeValue::Float32(v) => assert_eq!(v, 3.5),
            _ => panic!("expected Float32"),
        }
        match materialize_value(&Value::Float64(7.25), &mut store) {
            RuntimeValue::Float64(v) => assert_eq!(v, 7.25),
            _ => panic!("expected Float64"),
        }

        assert!(store.strings.is_empty());
        assert!(store.dynamic_integers.is_empty());
        assert!(store.structs.is_empty());
        assert!(store.enums.is_empty());
    }

    #[test]
    fn string_ownership_outlives_source() {
        let mut store = empty_store();
        let rt_string = {
            let temp_str = "hello dynamic string world".to_string();
            let val = Value::String(&temp_str);
            materialize_value(&val, &mut store)
        };

        match rt_string {
            RuntimeValue::String(StringBackingRef::Execution(id)) => {
                assert_eq!(id.0, 0);
                assert_eq!(&*store.strings[id.0], "hello dynamic string world");
            }
            _ => panic!("expected Execution string"),
        }
    }

    #[test]
    fn dynamic_integer_zero() {
        let mut store = empty_store();
        let val = Value::Dynamic(InterchangeDynamicValue::Integer(DynamicIntegerValue {
            negative: false,
            magnitude: Cow::Borrowed(&[]),
        }));
        let rt = materialize_value(&val, &mut store);
        match rt {
            RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                DynamicIntegerBackingRef::Execution(id),
            )) => {
                assert_eq!(id.0, 0);
                assert_eq!(store.dynamic_integers[id.0].value, BigInt::from(0));
            }
            _ => panic!("expected dynamic integer"),
        }
    }

    #[test]
    fn dynamic_integer_positive_and_negative() {
        let mut store = empty_store();

        // Positive 42 (0x2A)
        let pos_val = Value::Dynamic(InterchangeDynamicValue::Integer(DynamicIntegerValue {
            negative: false,
            magnitude: Cow::Borrowed(&[0x2A]),
        }));
        let rt_pos = materialize_value(&pos_val, &mut store);
        match rt_pos {
            RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                DynamicIntegerBackingRef::Execution(id),
            )) => {
                assert_eq!(id.0, 0);
                assert_eq!(store.dynamic_integers[id.0].value, BigInt::from(42));
            }
            _ => panic!("expected positive dynamic integer"),
        }

        // Negative 42
        let neg_val = Value::Dynamic(InterchangeDynamicValue::Integer(DynamicIntegerValue {
            negative: true,
            magnitude: Cow::Borrowed(&[0x2A]),
        }));
        let rt_neg = materialize_value(&neg_val, &mut store);
        match rt_neg {
            RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                DynamicIntegerBackingRef::Execution(id),
            )) => {
                assert_eq!(id.0, 1);
                assert_eq!(store.dynamic_integers[id.0].value, BigInt::from(-42));
            }
            _ => panic!("expected negative dynamic integer"),
        }
    }

    #[test]
    fn dynamic_integer_greater_than_u128() {
        let mut store = empty_store();
        // 2^128 = 1 followed by 16 zero bytes
        let mut mag = vec![0u8; 17];
        mag[0] = 1;

        let val = Value::Dynamic(InterchangeDynamicValue::Integer(DynamicIntegerValue {
            negative: false,
            magnitude: Cow::Owned(mag),
        }));
        let rt = materialize_value(&val, &mut store);
        match rt {
            RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                DynamicIntegerBackingRef::Execution(id),
            )) => {
                assert_eq!(
                    store.dynamic_integers[id.0].value.to_str_radix(10),
                    "340282366920938463463374607431768211456"
                );
            }
            _ => panic!("expected large dynamic integer"),
        }
    }

    #[test]
    fn dynamic_floats_inline_no_backing() {
        let mut store = empty_store();

        let f32_val = Value::Dynamic(InterchangeDynamicValue::Float32(1.25));
        let rt_f32 = materialize_value(&f32_val, &mut store);
        match rt_f32 {
            RuntimeValue::Dynamic(RuntimeDynamicValue::Float32(v)) => assert_eq!(v, 1.25),
            _ => panic!("expected Dynamic Float32"),
        }

        let f64_val = Value::Dynamic(InterchangeDynamicValue::Float64(9.875));
        let rt_f64 = materialize_value(&f64_val, &mut store);
        match rt_f64 {
            RuntimeValue::Dynamic(RuntimeDynamicValue::Float64(v)) => assert_eq!(v, 9.875),
            _ => panic!("expected Dynamic Float64"),
        }

        assert!(store.dynamic_integers.is_empty());
    }

    #[test]
    fn struct_materialization_and_field_order() {
        let mut store = empty_store();
        let val = Value::Struct(Box::new([
            Value::Int32(100),
            Value::String("field two"),
            Value::Dynamic(InterchangeDynamicValue::Integer(DynamicIntegerValue {
                negative: false,
                magnitude: Cow::Borrowed(&[7]),
            })),
        ]));

        let rt = materialize_value(&val, &mut store);
        match rt {
            RuntimeValue::Struct(id) => {
                assert_eq!(id.0, 0);
                let st = &store.structs[id.0];
                assert_eq!(st.fields.len(), 3);
                match st.fields[0] {
                    RuntimeValue::Int32(v) => assert_eq!(v, 100),
                    _ => panic!("expected field 0 Int32"),
                }
                match st.fields[1] {
                    RuntimeValue::String(StringBackingRef::Execution(sid)) => {
                        assert_eq!(&*store.strings[sid.0], "field two");
                    }
                    _ => panic!("expected field 1 String"),
                }
                match st.fields[2] {
                    RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                        DynamicIntegerBackingRef::Execution(did),
                    )) => {
                        assert_eq!(store.dynamic_integers[did.0].value, BigInt::from(7));
                    }
                    _ => panic!("expected field 2 Dynamic Integer"),
                }
            }
            _ => panic!("expected Struct"),
        }
    }

    #[test]
    fn nested_struct_child_appended_before_parent() {
        let mut store = empty_store();
        let inner = Value::Struct(Box::new([Value::Int32(10)]));
        let outer = Value::Struct(Box::new([inner, Value::Int32(20)]));

        let rt = materialize_value(&outer, &mut store);
        match rt {
            RuntimeValue::Struct(outer_id) => {
                assert_eq!(outer_id.0, 1); // Parent is inserted at index 1
                let outer_backing = &store.structs[outer_id.0];
                match outer_backing.fields[0] {
                    RuntimeValue::Struct(inner_id) => {
                        assert_eq!(inner_id.0, 0); // Child was inserted first at index 0
                        assert!(inner_id.0 < outer_id.0);
                    }
                    _ => panic!("expected nested Struct"),
                }
            }
            _ => panic!("expected outer Struct"),
        }
    }

    #[test]
    fn enum_simple() {
        let mut store = empty_store();
        let val = Value::Enum {
            variant: 3,
            payload: EnumPayload::Simple,
        };

        let rt = materialize_value(&val, &mut store);
        match rt {
            RuntimeValue::Enum(id) => {
                assert_eq!(id.0, 0);
                let eb = &store.enums[id.0];
                assert_eq!(eb.variant.0, 3);
                match eb.payload {
                    RuntimeEnumPayload::Simple => {}
                    _ => panic!("expected Simple payload"),
                }
            }
            _ => panic!("expected Enum"),
        }
    }

    #[test]
    fn enum_associated_child_before_parent() {
        let mut store = empty_store();
        let val = Value::Enum {
            variant: 1,
            payload: EnumPayload::Associated(Box::new(Value::String("associated payload"))),
        };

        let rt = materialize_value(&val, &mut store);
        match rt {
            RuntimeValue::Enum(id) => {
                assert_eq!(id.0, 0);
                let eb = &store.enums[id.0];
                assert_eq!(eb.variant.0, 1);
                match eb.payload {
                    RuntimeEnumPayload::Associated(RuntimeValue::String(
                        StringBackingRef::Execution(sid),
                    )) => {
                        assert_eq!(&*store.strings[sid.0], "associated payload");
                    }
                    _ => panic!("expected Associated String payload"),
                }
            }
            _ => panic!("expected Enum"),
        }
    }

    #[test]
    fn enum_structured_cardinality_and_order() {
        let mut store = empty_store();
        let val = Value::Enum {
            variant: 2,
            payload: EnumPayload::Structured {
                fields: Box::new([Value::Int32(1), Value::String("two")]),
            },
        };

        let rt = materialize_value(&val, &mut store);
        match rt {
            RuntimeValue::Enum(id) => {
                assert_eq!(id.0, 0);
                let eb = &store.enums[id.0];
                assert_eq!(eb.variant.0, 2);
                match &eb.payload {
                    RuntimeEnumPayload::Structured { fields } => {
                        assert_eq!(fields.len(), 2);
                        match fields[0] {
                            RuntimeValue::Int32(v) => assert_eq!(v, 1),
                            _ => panic!("expected field 0 Int32"),
                        }
                        match fields[1] {
                            RuntimeValue::String(StringBackingRef::Execution(sid)) => {
                                assert_eq!(&*store.strings[sid.0], "two");
                            }
                            _ => panic!("expected field 1 String"),
                        }
                    }
                    _ => panic!("expected Structured payload"),
                }
            }
            _ => panic!("expected Enum"),
        }
    }

    #[test]
    fn deep_composite_tree() {
        let mut store = empty_store();
        // Enum Structured { Struct([String]), Dynamic Integer }
        let val = Value::Enum {
            variant: 0,
            payload: EnumPayload::Structured {
                fields: Box::new([
                    Value::Struct(Box::new([Value::String("deep text")])),
                    Value::Dynamic(InterchangeDynamicValue::Integer(DynamicIntegerValue {
                        negative: true,
                        magnitude: Cow::Borrowed(&[99]),
                    })),
                ]),
            },
        };

        let rt = materialize_value(&val, &mut store);
        match rt {
            RuntimeValue::Enum(enum_id) => {
                assert_eq!(enum_id.0, 0);
                let eb = &store.enums[enum_id.0];
                match &eb.payload {
                    RuntimeEnumPayload::Structured { fields } => {
                        assert_eq!(fields.len(), 2);
                        match fields[0] {
                            RuntimeValue::Struct(st_id) => {
                                assert_eq!(st_id.0, 0);
                                let st = &store.structs[st_id.0];
                                match st.fields[0] {
                                    RuntimeValue::String(StringBackingRef::Execution(s_id)) => {
                                        assert_eq!(&*store.strings[s_id.0], "deep text");
                                    }
                                    _ => panic!("expected deep String"),
                                }
                            }
                            _ => panic!("expected Struct field"),
                        }
                        match fields[1] {
                            RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                                DynamicIntegerBackingRef::Execution(did),
                            )) => {
                                assert_eq!(store.dynamic_integers[did.0].value, BigInt::from(-99));
                            }
                            _ => panic!("expected Dynamic Integer field"),
                        }
                    }
                    _ => panic!("expected Structured payload"),
                }
            }
            _ => panic!("expected root Enum"),
        }
    }
}
