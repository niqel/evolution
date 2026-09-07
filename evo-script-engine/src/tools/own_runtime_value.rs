use alloc::boxed::Box;
use alloc::vec::Vec;
use num_bigint::Sign;

use crate::data::compiled::program::CompiledProgram;
use crate::data::compiled::storage::{Constant, DynamicConstant};
use crate::data::vm::backing::{ExecutionBackingStore, RuntimeEnumPayload};
use crate::data::vm::values::{
    DynamicIntegerBackingRef, DynamicValue as RuntimeDynamicValue, RuntimeValue, StringBackingRef,
};
use evo_values::{OwnedDynamicInteger, OwnedDynamicValue, OwnedEnumPayload, OwnedValue};

pub type OwnRuntimeValue = fn(RuntimeValue, &CompiledProgram, &ExecutionBackingStore) -> OwnedValue;

pub fn own_runtime_value(
    value: RuntimeValue,
    compiled_program: &CompiledProgram,
    backing_store: &ExecutionBackingStore,
) -> OwnedValue {
    match value {
        RuntimeValue::Boolean(b) => OwnedValue::Boolean(b),

        RuntimeValue::Int8(v) => OwnedValue::Int8(v),
        RuntimeValue::Int16(v) => OwnedValue::Int16(v),
        RuntimeValue::Int32(v) => OwnedValue::Int32(v),
        RuntimeValue::Int64(v) => OwnedValue::Int64(v),
        RuntimeValue::Int128(v) => OwnedValue::Int128(v),

        RuntimeValue::Uint8(v) => OwnedValue::Uint8(v),
        RuntimeValue::Uint16(v) => OwnedValue::Uint16(v),
        RuntimeValue::Uint32(v) => OwnedValue::Uint32(v),
        RuntimeValue::Uint64(v) => OwnedValue::Uint64(v),
        RuntimeValue::Uint128(v) => OwnedValue::Uint128(v),

        RuntimeValue::Float32(v) => OwnedValue::Float32(v),
        RuntimeValue::Float64(v) => OwnedValue::Float64(v),

        RuntimeValue::String(string_ref) => match string_ref {
            StringBackingRef::Compiled(constant_id) => {
                let constant = compiled_program
                    .constants
                    .get(constant_id.0)
                    .expect("ConstantId must reference compiled program constant pool");
                match constant {
                    Constant::String(s) => OwnedValue::String(s.as_str().into()),
                    _ => panic!("Expected Constant::String at constant id"),
                }
            }
            StringBackingRef::Execution(id) => {
                let s = backing_store
                    .strings
                    .get(id.0)
                    .expect("StringBackingId must reference execution backing store");
                OwnedValue::String(s.as_ref().into())
            }
        },

        RuntimeValue::Dynamic(dyn_val) => match dyn_val {
            RuntimeDynamicValue::Float32(v) => OwnedValue::Dynamic(OwnedDynamicValue::Float32(v)),
            RuntimeDynamicValue::Float64(v) => OwnedValue::Dynamic(OwnedDynamicValue::Float64(v)),
            RuntimeDynamicValue::Integer(int_ref) => match int_ref {
                DynamicIntegerBackingRef::Compiled(constant_id) => {
                    let constant = compiled_program
                        .constants
                        .get(constant_id.0)
                        .expect("ConstantId must reference compiled program constant pool");
                    match constant {
                        Constant::Dynamic(DynamicConstant::Integer {
                            negative,
                            magnitude,
                        }) => OwnedValue::Dynamic(OwnedDynamicValue::Integer(
                            OwnedDynamicInteger::from_parts(*negative, magnitude.as_slice().into()),
                        )),
                        _ => panic!("Expected Constant::Dynamic(Integer) at constant id"),
                    }
                }
                DynamicIntegerBackingRef::Execution(id) => {
                    let backing = backing_store
                        .dynamic_integers
                        .get(id.0)
                        .expect("DynamicIntegerBackingId must reference execution backing store");
                    let (sign, magnitude) = backing.value.to_bytes_be();
                    let (negative, magnitude) = match sign {
                        Sign::Minus => (true, magnitude.into_boxed_slice()),
                        Sign::Plus => (false, magnitude.into_boxed_slice()),
                        Sign::NoSign => (false, vec![].into_boxed_slice()),
                    };
                    OwnedValue::Dynamic(OwnedDynamicValue::Integer(
                        OwnedDynamicInteger::from_parts(negative, magnitude),
                    ))
                }
            },
        },

        RuntimeValue::Struct(struct_id) => {
            let backing = backing_store
                .structs
                .get(struct_id.0)
                .expect("StructBackingId must reference execution backing store");
            let mut owned_fields = Vec::with_capacity(backing.fields.len());
            for &field in backing.fields.iter() {
                owned_fields.push(own_runtime_value(field, compiled_program, backing_store));
            }
            OwnedValue::Struct(owned_fields.into_boxed_slice())
        }

        RuntimeValue::Enum(enum_id) => {
            let backing = backing_store
                .enums
                .get(enum_id.0)
                .expect("EnumBackingId must reference execution backing store");
            let owned_payload = match &backing.payload {
                RuntimeEnumPayload::Simple => OwnedEnumPayload::Simple,
                RuntimeEnumPayload::Associated(associated_val) => {
                    let owned_val =
                        own_runtime_value(*associated_val, compiled_program, backing_store);
                    OwnedEnumPayload::Associated(Box::new(owned_val))
                }
                RuntimeEnumPayload::Structured { fields } => {
                    let mut owned_fields = Vec::with_capacity(fields.len());
                    for &field in fields.iter() {
                        owned_fields.push(own_runtime_value(
                            field,
                            compiled_program,
                            backing_store,
                        ));
                    }
                    OwnedEnumPayload::Structured {
                        fields: owned_fields.into_boxed_slice(),
                    }
                }
            };
            OwnedValue::Enum {
                variant: backing.variant.0,
                payload: owned_payload,
            }
        }
    }
}

pub const OWN_RUNTIME_VALUE: OwnRuntimeValue = own_runtime_value;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use alloc::vec;
    use num_bigint::BigInt;

    use crate::data::compiled::identities::{ConstantId, VariantDiscriminant};
    use crate::data::compiled::source_map::SourceMap;
    use crate::data::semantic::ids::FunctionId;
    use crate::data::vm::backing::{DynamicIntegerBacking, EnumBacking, StructBacking};
    use crate::data::vm::values::{
        DynamicIntegerBackingId, EnumBackingId, StringBackingId, StructBackingId,
    };

    fn empty_program() -> CompiledProgram {
        CompiledProgram {
            functions: Vec::new(),
            entry_point: FunctionId(0),
            entry_parameter_shapes: Vec::new(),
            constants: Vec::new(),
            external_symbols: Vec::new(),
            value_shapes: Vec::new(),
            source_map: SourceMap {
                functions: Vec::new(),
            },
        }
    }

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
        let implementation: OwnRuntimeValue = own_runtime_value;
        let binding: OwnRuntimeValue = OWN_RUNTIME_VALUE;
        assert_eq!(implementation as usize, binding as usize);
    }

    #[test]
    fn fixed_scalars() {
        let program = empty_program();
        let store = empty_store();

        match own_runtime_value(RuntimeValue::Boolean(true), &program, &store) {
            OwnedValue::Boolean(b) => assert!(b),
            _ => panic!("expected Boolean"),
        }
        match own_runtime_value(RuntimeValue::Int8(-8), &program, &store) {
            OwnedValue::Int8(v) => assert_eq!(v, -8),
            _ => panic!("expected Int8"),
        }
        match own_runtime_value(RuntimeValue::Int16(-16), &program, &store) {
            OwnedValue::Int16(v) => assert_eq!(v, -16),
            _ => panic!("expected Int16"),
        }
        match own_runtime_value(RuntimeValue::Int32(-32), &program, &store) {
            OwnedValue::Int32(v) => assert_eq!(v, -32),
            _ => panic!("expected Int32"),
        }
        match own_runtime_value(RuntimeValue::Int64(-64), &program, &store) {
            OwnedValue::Int64(v) => assert_eq!(v, -64),
            _ => panic!("expected Int64"),
        }
        match own_runtime_value(RuntimeValue::Int128(-128), &program, &store) {
            OwnedValue::Int128(v) => assert_eq!(v, -128),
            _ => panic!("expected Int128"),
        }
        match own_runtime_value(RuntimeValue::Uint8(8), &program, &store) {
            OwnedValue::Uint8(v) => assert_eq!(v, 8),
            _ => panic!("expected Uint8"),
        }
        match own_runtime_value(RuntimeValue::Uint16(16), &program, &store) {
            OwnedValue::Uint16(v) => assert_eq!(v, 16),
            _ => panic!("expected Uint16"),
        }
        match own_runtime_value(RuntimeValue::Uint32(32), &program, &store) {
            OwnedValue::Uint32(v) => assert_eq!(v, 32),
            _ => panic!("expected Uint32"),
        }
        match own_runtime_value(RuntimeValue::Uint64(64), &program, &store) {
            OwnedValue::Uint64(v) => assert_eq!(v, 64),
            _ => panic!("expected Uint64"),
        }
        match own_runtime_value(RuntimeValue::Uint128(128), &program, &store) {
            OwnedValue::Uint128(v) => assert_eq!(v, 128),
            _ => panic!("expected Uint128"),
        }
        match own_runtime_value(RuntimeValue::Float32(1.5), &program, &store) {
            OwnedValue::Float32(v) => assert_eq!(v, 1.5),
            _ => panic!("expected Float32"),
        }
        match own_runtime_value(RuntimeValue::Float64(2.5), &program, &store) {
            OwnedValue::Float64(v) => assert_eq!(v, 2.5),
            _ => panic!("expected Float64"),
        }
    }

    #[test]
    fn compiled_string() {
        let mut program = empty_program();
        program
            .constants
            .push(Constant::String("compiled string".to_string()));
        let store = empty_store();

        let val = RuntimeValue::String(StringBackingRef::Compiled(ConstantId(0)));
        let owned = own_runtime_value(val, &program, &store);
        match owned {
            OwnedValue::String(s) => assert_eq!(&*s, "compiled string"),
            _ => panic!("expected String"),
        }
    }

    #[test]
    fn execution_string() {
        let program = empty_program();
        let mut store = empty_store();
        store
            .strings
            .push("execution string".to_string().into_boxed_str());

        let val = RuntimeValue::String(StringBackingRef::Execution(StringBackingId(0)));
        let owned = own_runtime_value(val, &program, &store);
        match owned {
            OwnedValue::String(s) => assert_eq!(&*s, "execution string"),
            _ => panic!("expected String"),
        }
    }

    #[test]
    fn compiled_dynamic_integer() {
        let mut program = empty_program();
        // 0: zero
        program
            .constants
            .push(Constant::Dynamic(DynamicConstant::Integer {
                negative: false,
                magnitude: vec![],
            }));
        // 1: positive 42
        program
            .constants
            .push(Constant::Dynamic(DynamicConstant::Integer {
                negative: false,
                magnitude: vec![42],
            }));
        // 2: negative 42
        program
            .constants
            .push(Constant::Dynamic(DynamicConstant::Integer {
                negative: true,
                magnitude: vec![42],
            }));
        let store = empty_store();

        let val_zero = RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
            DynamicIntegerBackingRef::Compiled(ConstantId(0)),
        ));
        match own_runtime_value(val_zero, &program, &store) {
            OwnedValue::Dynamic(OwnedDynamicValue::Integer(dyn_int)) => {
                assert!(!dyn_int.negative());
                assert_eq!(dyn_int.magnitude(), &[] as &[u8]);
            }
            _ => panic!("expected dynamic integer"),
        }

        let val_pos = RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
            DynamicIntegerBackingRef::Compiled(ConstantId(1)),
        ));
        match own_runtime_value(val_pos, &program, &store) {
            OwnedValue::Dynamic(OwnedDynamicValue::Integer(dyn_int)) => {
                assert!(!dyn_int.negative());
                assert_eq!(dyn_int.magnitude(), &[42]);
            }
            _ => panic!("expected dynamic integer"),
        }

        let val_neg = RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
            DynamicIntegerBackingRef::Compiled(ConstantId(2)),
        ));
        match own_runtime_value(val_neg, &program, &store) {
            OwnedValue::Dynamic(OwnedDynamicValue::Integer(dyn_int)) => {
                assert!(dyn_int.negative());
                assert_eq!(dyn_int.magnitude(), &[42]);
            }
            _ => panic!("expected dynamic integer"),
        }
    }

    #[test]
    fn execution_dynamic_integer() {
        let program = empty_program();
        let mut store = empty_store();
        // 0: zero
        store.dynamic_integers.push(DynamicIntegerBacking {
            value: BigInt::from(0),
        });
        // 1: positive 42
        store.dynamic_integers.push(DynamicIntegerBacking {
            value: BigInt::from(42),
        });
        // 2: negative 42
        store.dynamic_integers.push(DynamicIntegerBacking {
            value: BigInt::from(-42),
        });
        // 3: > u128 (2^128 = 340282366920938463463374607431768211456)
        let large_bigint = BigInt::parse_bytes(b"340282366920938463463374607431768211456", 10)
            .expect("valid decimal");
        store.dynamic_integers.push(DynamicIntegerBacking {
            value: large_bigint,
        });

        let val_zero = RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
            DynamicIntegerBackingRef::Execution(DynamicIntegerBackingId(0)),
        ));
        match own_runtime_value(val_zero, &program, &store) {
            OwnedValue::Dynamic(OwnedDynamicValue::Integer(dyn_int)) => {
                assert!(!dyn_int.negative());
                assert_eq!(dyn_int.magnitude(), &[] as &[u8]);
            }
            _ => panic!("expected dynamic integer"),
        }

        let val_pos = RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
            DynamicIntegerBackingRef::Execution(DynamicIntegerBackingId(1)),
        ));
        match own_runtime_value(val_pos, &program, &store) {
            OwnedValue::Dynamic(OwnedDynamicValue::Integer(dyn_int)) => {
                assert!(!dyn_int.negative());
                assert_eq!(dyn_int.magnitude(), &[42]);
            }
            _ => panic!("expected dynamic integer"),
        }

        let val_neg = RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
            DynamicIntegerBackingRef::Execution(DynamicIntegerBackingId(2)),
        ));
        match own_runtime_value(val_neg, &program, &store) {
            OwnedValue::Dynamic(OwnedDynamicValue::Integer(dyn_int)) => {
                assert!(dyn_int.negative());
                assert_eq!(dyn_int.magnitude(), &[42]);
            }
            _ => panic!("expected dynamic integer"),
        }

        let val_large = RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
            DynamicIntegerBackingRef::Execution(DynamicIntegerBackingId(3)),
        ));
        let mut expected_large_mag = vec![0u8; 17];
        expected_large_mag[0] = 1;
        match own_runtime_value(val_large, &program, &store) {
            OwnedValue::Dynamic(OwnedDynamicValue::Integer(dyn_int)) => {
                assert!(!dyn_int.negative());
                assert_eq!(dyn_int.magnitude(), expected_large_mag.as_slice());
            }
            _ => panic!("expected dynamic integer"),
        }
    }

    #[test]
    fn dynamic_floats() {
        let program = empty_program();
        let store = empty_store();

        let val_f32 = RuntimeValue::Dynamic(RuntimeDynamicValue::Float32(1.25));
        match own_runtime_value(val_f32, &program, &store) {
            OwnedValue::Dynamic(OwnedDynamicValue::Float32(v)) => assert_eq!(v, 1.25),
            _ => panic!("expected Float32"),
        }

        let val_f64 = RuntimeValue::Dynamic(RuntimeDynamicValue::Float64(9.875));
        match own_runtime_value(val_f64, &program, &store) {
            OwnedValue::Dynamic(OwnedDynamicValue::Float64(v)) => assert_eq!(v, 9.875),
            _ => panic!("expected Float64"),
        }
    }

    #[test]
    fn struct_ownership() {
        let program = empty_program();
        let mut store = empty_store();
        store
            .strings
            .push("struct field text".to_string().into_boxed_str());
        store.dynamic_integers.push(DynamicIntegerBacking {
            value: BigInt::from(7),
        });
        store.structs.push(StructBacking {
            fields: vec![
                RuntimeValue::Int32(100),
                RuntimeValue::String(StringBackingRef::Execution(StringBackingId(0))),
                RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                    DynamicIntegerBackingRef::Execution(DynamicIntegerBackingId(0)),
                )),
            ]
            .into_boxed_slice(),
        });

        let val = RuntimeValue::Struct(StructBackingId(0));
        let owned = own_runtime_value(val, &program, &store);
        match owned {
            OwnedValue::Struct(fields) => {
                assert_eq!(fields.len(), 3);
                match &fields[0] {
                    OwnedValue::Int32(v) => assert_eq!(*v, 100),
                    _ => panic!("expected Int32"),
                }
                match &fields[1] {
                    OwnedValue::String(s) => assert_eq!(&**s, "struct field text"),
                    _ => panic!("expected String"),
                }
                match &fields[2] {
                    OwnedValue::Dynamic(OwnedDynamicValue::Integer(dyn_int)) => {
                        assert!(!dyn_int.negative());
                        assert_eq!(dyn_int.magnitude(), &[7]);
                    }
                    _ => panic!("expected dynamic integer"),
                }
            }
            _ => panic!("expected Struct"),
        }
    }

    #[test]
    fn nested_struct() {
        let program = empty_program();
        let mut store = empty_store();
        store
            .strings
            .push("nested string".to_string().into_boxed_str());
        // Inner struct at 0
        store.structs.push(StructBacking {
            fields: vec![RuntimeValue::String(StringBackingRef::Execution(
                StringBackingId(0),
            ))]
            .into_boxed_slice(),
        });
        // Outer struct at 1
        store.structs.push(StructBacking {
            fields: vec![
                RuntimeValue::Int32(42),
                RuntimeValue::Struct(StructBackingId(0)),
            ]
            .into_boxed_slice(),
        });

        let val = RuntimeValue::Struct(StructBackingId(1));
        let owned = own_runtime_value(val, &program, &store);
        match owned {
            OwnedValue::Struct(fields) => {
                assert_eq!(fields.len(), 2);
                match &fields[0] {
                    OwnedValue::Int32(v) => assert_eq!(*v, 42),
                    _ => panic!("expected Int32"),
                }
                match &fields[1] {
                    OwnedValue::Struct(inner_fields) => {
                        assert_eq!(inner_fields.len(), 1);
                        match &inner_fields[0] {
                            OwnedValue::String(s) => assert_eq!(&**s, "nested string"),
                            _ => panic!("expected String"),
                        }
                    }
                    _ => panic!("expected nested Struct"),
                }
            }
            _ => panic!("expected Struct"),
        }
    }

    #[test]
    fn enum_simple() {
        let program = empty_program();
        let mut store = empty_store();
        store.enums.push(EnumBacking {
            variant: VariantDiscriminant(5),
            payload: RuntimeEnumPayload::Simple,
        });

        let val = RuntimeValue::Enum(EnumBackingId(0));
        let owned = own_runtime_value(val, &program, &store);
        match owned {
            OwnedValue::Enum { variant, payload } => {
                assert_eq!(variant, 5);
                match payload {
                    OwnedEnumPayload::Simple => {}
                    _ => panic!("expected Simple payload"),
                }
            }
            _ => panic!("expected Enum"),
        }
    }

    #[test]
    fn enum_associated() {
        let program = empty_program();
        let mut store = empty_store();
        store
            .strings
            .push("associated msg".to_string().into_boxed_str());
        store.enums.push(EnumBacking {
            variant: VariantDiscriminant(1),
            payload: RuntimeEnumPayload::Associated(RuntimeValue::String(
                StringBackingRef::Execution(StringBackingId(0)),
            )),
        });

        let val = RuntimeValue::Enum(EnumBackingId(0));
        let owned = own_runtime_value(val, &program, &store);
        match owned {
            OwnedValue::Enum { variant, payload } => {
                assert_eq!(variant, 1);
                match payload {
                    OwnedEnumPayload::Associated(inner) => match *inner {
                        OwnedValue::String(s) => assert_eq!(&*s, "associated msg"),
                        _ => panic!("expected String"),
                    },
                    _ => panic!("expected Associated payload"),
                }
            }
            _ => panic!("expected Enum"),
        }
    }

    #[test]
    fn enum_structured() {
        let program = empty_program();
        let mut store = empty_store();
        store.strings.push("payload".to_string().into_boxed_str());
        // Non-empty structured at 0
        store.enums.push(EnumBacking {
            variant: VariantDiscriminant(2),
            payload: RuntimeEnumPayload::Structured {
                fields: vec![
                    RuntimeValue::Int32(1),
                    RuntimeValue::String(StringBackingRef::Execution(StringBackingId(0))),
                ]
                .into_boxed_slice(),
            },
        });
        // Empty structured at 1
        store.enums.push(EnumBacking {
            variant: VariantDiscriminant(3),
            payload: RuntimeEnumPayload::Structured {
                fields: vec![].into_boxed_slice(),
            },
        });

        let val_non_empty = RuntimeValue::Enum(EnumBackingId(0));
        match own_runtime_value(val_non_empty, &program, &store) {
            OwnedValue::Enum { variant, payload } => {
                assert_eq!(variant, 2);
                match payload {
                    OwnedEnumPayload::Structured { fields } => {
                        assert_eq!(fields.len(), 2);
                        match &fields[0] {
                            OwnedValue::Int32(v) => assert_eq!(*v, 1),
                            _ => panic!("expected Int32"),
                        }
                        match &fields[1] {
                            OwnedValue::String(s) => assert_eq!(&**s, "payload"),
                            _ => panic!("expected String"),
                        }
                    }
                    _ => panic!("expected Structured payload"),
                }
            }
            _ => panic!("expected Enum"),
        }

        let val_empty = RuntimeValue::Enum(EnumBackingId(1));
        match own_runtime_value(val_empty, &program, &store) {
            OwnedValue::Enum { variant, payload } => {
                assert_eq!(variant, 3);
                match payload {
                    OwnedEnumPayload::Structured { fields } => assert_eq!(fields.len(), 0),
                    _ => panic!("expected Structured payload"),
                }
            }
            _ => panic!("expected Enum"),
        }
    }

    #[test]
    fn deep_composite_tree() {
        let program = empty_program();
        let mut store = empty_store();
        store.strings.push("deep leaf".to_string().into_boxed_str());
        store.dynamic_integers.push(DynamicIntegerBacking {
            value: BigInt::from(-99),
        });
        store.structs.push(StructBacking {
            fields: vec![RuntimeValue::String(StringBackingRef::Execution(
                StringBackingId(0),
            ))]
            .into_boxed_slice(),
        });
        store.enums.push(EnumBacking {
            variant: VariantDiscriminant(0),
            payload: RuntimeEnumPayload::Structured {
                fields: vec![
                    RuntimeValue::Struct(StructBackingId(0)),
                    RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                        DynamicIntegerBackingRef::Execution(DynamicIntegerBackingId(0)),
                    )),
                ]
                .into_boxed_slice(),
            },
        });

        let val = RuntimeValue::Enum(EnumBackingId(0));
        let owned = own_runtime_value(val, &program, &store);
        match owned {
            OwnedValue::Enum { variant, payload } => {
                assert_eq!(variant, 0);
                match payload {
                    OwnedEnumPayload::Structured { fields } => {
                        assert_eq!(fields.len(), 2);
                        match &fields[0] {
                            OwnedValue::Struct(st_fields) => {
                                assert_eq!(st_fields.len(), 1);
                                match &st_fields[0] {
                                    OwnedValue::String(s) => assert_eq!(&**s, "deep leaf"),
                                    _ => panic!("expected String"),
                                }
                            }
                            _ => panic!("expected Struct"),
                        }
                        match &fields[1] {
                            OwnedValue::Dynamic(OwnedDynamicValue::Integer(dyn_int)) => {
                                assert!(dyn_int.negative());
                                assert_eq!(dyn_int.magnitude(), &[99]);
                            }
                            _ => panic!("expected dynamic integer"),
                        }
                    }
                    _ => panic!("expected Structured payload"),
                }
            }
            _ => panic!("expected Enum"),
        }
    }

    #[test]
    #[should_panic(expected = "StringBackingId must reference execution backing store")]
    fn dangling_execution_backing_id_is_invariant_violation() {
        let program = empty_program();
        let store = empty_store();
        let val = RuntimeValue::String(StringBackingRef::Execution(StringBackingId(99)));
        own_runtime_value(val, &program, &store);
    }

    #[test]
    #[should_panic(expected = "Expected Constant::String at constant id")]
    fn wrong_constant_family_for_compiled_string_is_invariant_violation() {
        let mut program = empty_program();
        program.constants.push(Constant::Int32(42));
        let store = empty_store();
        let val = RuntimeValue::String(StringBackingRef::Compiled(ConstantId(0)));
        own_runtime_value(val, &program, &store);
    }

    #[test]
    #[should_panic(expected = "Expected Constant::Dynamic(Integer) at constant id")]
    fn wrong_constant_family_for_compiled_dynamic_int_is_invariant_violation() {
        let mut program = empty_program();
        program.constants.push(Constant::Boolean(true));
        let store = empty_store();
        let val = RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
            DynamicIntegerBackingRef::Compiled(ConstantId(0)),
        ));
        own_runtime_value(val, &program, &store);
    }
}
