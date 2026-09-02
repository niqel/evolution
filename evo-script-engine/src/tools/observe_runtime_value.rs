use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::vec::Vec;
use num_bigint::Sign;

use crate::data::compiled::program::CompiledProgram;
use crate::data::compiled::storage::{Constant, DynamicConstant};
use crate::data::vm::backing::{ExecutionBackingStore, RuntimeEnumPayload};
use crate::data::vm::values::{
    DynamicIntegerBackingRef, DynamicValue as RuntimeDynamicValue, RuntimeValue, StringBackingRef,
};
use evo_values::{
    DynamicIntegerValue, DynamicValue as InterchangeDynamicValue, EnumPayload, Value,
};

pub type ObserveRuntimeValue = for<'value> fn(
    RuntimeValue,
    &'value CompiledProgram,
    &'value ExecutionBackingStore,
) -> Value<'value>;

pub fn observe_runtime_value<'value>(
    value: RuntimeValue,
    compiled_program: &'value CompiledProgram,
    backing_store: &'value ExecutionBackingStore,
) -> Value<'value> {
    match value {
        RuntimeValue::Boolean(b) => Value::Boolean(b),

        RuntimeValue::Int8(v) => Value::Int8(v),
        RuntimeValue::Int16(v) => Value::Int16(v),
        RuntimeValue::Int32(v) => Value::Int32(v),
        RuntimeValue::Int64(v) => Value::Int64(v),
        RuntimeValue::Int128(v) => Value::Int128(v),

        RuntimeValue::Uint8(v) => Value::Uint8(v),
        RuntimeValue::Uint16(v) => Value::Uint16(v),
        RuntimeValue::Uint32(v) => Value::Uint32(v),
        RuntimeValue::Uint64(v) => Value::Uint64(v),
        RuntimeValue::Uint128(v) => Value::Uint128(v),

        RuntimeValue::Float32(v) => Value::Float32(v),
        RuntimeValue::Float64(v) => Value::Float64(v),

        RuntimeValue::String(string_ref) => match string_ref {
            StringBackingRef::Compiled(constant_id) => {
                let constant = compiled_program
                    .constants
                    .get(constant_id.0)
                    .expect("ConstantId must reference compiled program constant pool");
                match constant {
                    Constant::String(s) => Value::String(s.as_str()),
                    _ => panic!("Expected Constant::String at constant id"),
                }
            }
            StringBackingRef::Execution(id) => {
                let s = backing_store
                    .strings
                    .get(id.0)
                    .expect("StringBackingId must reference execution backing store");
                Value::String(s.as_ref())
            }
        },

        RuntimeValue::Dynamic(dyn_val) => match dyn_val {
            RuntimeDynamicValue::Float32(v) => Value::Dynamic(InterchangeDynamicValue::Float32(v)),
            RuntimeDynamicValue::Float64(v) => Value::Dynamic(InterchangeDynamicValue::Float64(v)),
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
                        }) => {
                            Value::Dynamic(InterchangeDynamicValue::Integer(DynamicIntegerValue {
                                negative: *negative,
                                magnitude: Cow::Borrowed(magnitude.as_slice()),
                            }))
                        }
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
                        Sign::Minus => (true, magnitude),
                        Sign::Plus => (false, magnitude),
                        Sign::NoSign => (false, Vec::new()),
                    };
                    Value::Dynamic(InterchangeDynamicValue::Integer(DynamicIntegerValue {
                        negative,
                        magnitude: Cow::Owned(magnitude),
                    }))
                }
            },
        },

        RuntimeValue::Struct(struct_id) => {
            let backing = backing_store
                .structs
                .get(struct_id.0)
                .expect("StructBackingId must reference execution backing store");
            let mut observed_fields = Vec::with_capacity(backing.fields.len());
            for &field in backing.fields.iter() {
                observed_fields.push(observe_runtime_value(
                    field,
                    compiled_program,
                    backing_store,
                ));
            }
            Value::Struct(observed_fields.into_boxed_slice())
        }

        RuntimeValue::Enum(enum_id) => {
            let backing = backing_store
                .enums
                .get(enum_id.0)
                .expect("EnumBackingId must reference execution backing store");
            let payload = match &backing.payload {
                RuntimeEnumPayload::Simple => EnumPayload::Simple,
                RuntimeEnumPayload::Associated(associated_val) => {
                    let observed_val =
                        observe_runtime_value(*associated_val, compiled_program, backing_store);
                    EnumPayload::Associated(Box::new(observed_val))
                }
                RuntimeEnumPayload::Structured { fields } => {
                    let mut observed_fields = Vec::with_capacity(fields.len());
                    for &field in fields.iter() {
                        observed_fields.push(observe_runtime_value(
                            field,
                            compiled_program,
                            backing_store,
                        ));
                    }
                    EnumPayload::Structured {
                        fields: observed_fields.into_boxed_slice(),
                    }
                }
            };
            Value::Enum {
                variant: backing.variant.0,
                payload,
            }
        }
    }
}

pub const OBSERVE_RUNTIME_VALUE: ObserveRuntimeValue = observe_runtime_value;

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
        let implementation: ObserveRuntimeValue = observe_runtime_value;
        let binding: ObserveRuntimeValue = OBSERVE_RUNTIME_VALUE;
        assert_eq!(implementation as usize, binding as usize);
    }

    #[test]
    fn fixed_scalars() {
        let program = empty_program();
        let store = empty_store();

        assert_eq!(
            observe_runtime_value(RuntimeValue::Boolean(true), &program, &store),
            Value::Boolean(true)
        );
        assert_eq!(
            observe_runtime_value(RuntimeValue::Int8(-8), &program, &store),
            Value::Int8(-8)
        );
        assert_eq!(
            observe_runtime_value(RuntimeValue::Int16(-16), &program, &store),
            Value::Int16(-16)
        );
        assert_eq!(
            observe_runtime_value(RuntimeValue::Int32(-32), &program, &store),
            Value::Int32(-32)
        );
        assert_eq!(
            observe_runtime_value(RuntimeValue::Int64(-64), &program, &store),
            Value::Int64(-64)
        );
        assert_eq!(
            observe_runtime_value(RuntimeValue::Int128(-128), &program, &store),
            Value::Int128(-128)
        );
        assert_eq!(
            observe_runtime_value(RuntimeValue::Uint8(8), &program, &store),
            Value::Uint8(8)
        );
        assert_eq!(
            observe_runtime_value(RuntimeValue::Uint16(16), &program, &store),
            Value::Uint16(16)
        );
        assert_eq!(
            observe_runtime_value(RuntimeValue::Uint32(32), &program, &store),
            Value::Uint32(32)
        );
        assert_eq!(
            observe_runtime_value(RuntimeValue::Uint64(64), &program, &store),
            Value::Uint64(64)
        );
        assert_eq!(
            observe_runtime_value(RuntimeValue::Uint128(128), &program, &store),
            Value::Uint128(128)
        );
        assert_eq!(
            observe_runtime_value(RuntimeValue::Float32(1.5), &program, &store),
            Value::Float32(1.5)
        );
        assert_eq!(
            observe_runtime_value(RuntimeValue::Float64(2.5), &program, &store),
            Value::Float64(2.5)
        );
    }

    #[test]
    fn compiled_string_borrows() {
        let mut program = empty_program();
        program
            .constants
            .push(Constant::String("compiled string".to_string()));
        let store = empty_store();

        let val = RuntimeValue::String(StringBackingRef::Compiled(ConstantId(0)));
        let observed = observe_runtime_value(val, &program, &store);
        match observed {
            Value::String(s) => {
                assert_eq!(s, "compiled string");
                if let Constant::String(ref orig) = program.constants[0] {
                    assert_eq!(s.as_ptr(), orig.as_ptr());
                    assert_eq!(s.len(), orig.len());
                } else {
                    panic!("expected Constant::String");
                }
            }
            _ => panic!("expected Value::String"),
        }
    }

    #[test]
    fn execution_string_borrows() {
        let program = empty_program();
        let mut store = empty_store();
        store
            .strings
            .push("execution string".to_string().into_boxed_str());

        let val = RuntimeValue::String(StringBackingRef::Execution(StringBackingId(0)));
        let observed = observe_runtime_value(val, &program, &store);
        match observed {
            Value::String(s) => {
                assert_eq!(s, "execution string");
                let orig = &store.strings[0];
                assert_eq!(s.as_ptr(), orig.as_ptr());
                assert_eq!(s.len(), orig.len());
            }
            _ => panic!("expected Value::String"),
        }
    }

    #[test]
    fn compiled_dynamic_integer_uses_cow_borrowed() {
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

        // 0: zero
        let val_zero = RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
            DynamicIntegerBackingRef::Compiled(ConstantId(0)),
        ));
        let obs_zero = observe_runtime_value(val_zero, &program, &store);
        match obs_zero {
            Value::Dynamic(InterchangeDynamicValue::Integer(dyn_int)) => {
                assert!(!dyn_int.negative);
                assert!(matches!(dyn_int.magnitude, Cow::Borrowed(_)));
                assert_eq!(&*dyn_int.magnitude, &[] as &[u8]);
            }
            _ => panic!("expected dynamic integer"),
        }

        // 1: positive 42
        let val_pos = RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
            DynamicIntegerBackingRef::Compiled(ConstantId(1)),
        ));
        let obs_pos = observe_runtime_value(val_pos, &program, &store);
        match obs_pos {
            Value::Dynamic(InterchangeDynamicValue::Integer(dyn_int)) => {
                assert!(!dyn_int.negative);
                assert!(matches!(dyn_int.magnitude, Cow::Borrowed(_)));
                assert_eq!(&*dyn_int.magnitude, &[42]);
            }
            _ => panic!("expected dynamic integer"),
        }

        // 2: negative 42
        let val_neg = RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
            DynamicIntegerBackingRef::Compiled(ConstantId(2)),
        ));
        let obs_neg = observe_runtime_value(val_neg, &program, &store);
        match obs_neg {
            Value::Dynamic(InterchangeDynamicValue::Integer(dyn_int)) => {
                assert!(dyn_int.negative);
                assert!(matches!(dyn_int.magnitude, Cow::Borrowed(_)));
                assert_eq!(&*dyn_int.magnitude, &[42]);
            }
            _ => panic!("expected dynamic integer"),
        }
    }

    #[test]
    fn execution_dynamic_integer_uses_cow_owned() {
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

        // 0: zero
        let val_zero = RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
            DynamicIntegerBackingRef::Execution(DynamicIntegerBackingId(0)),
        ));
        let obs_zero = observe_runtime_value(val_zero, &program, &store);
        match obs_zero {
            Value::Dynamic(InterchangeDynamicValue::Integer(dyn_int)) => {
                assert!(!dyn_int.negative);
                assert!(matches!(dyn_int.magnitude, Cow::Owned(_)));
                assert_eq!(&*dyn_int.magnitude, &[] as &[u8]);
            }
            _ => panic!("expected dynamic integer"),
        }

        // 1: positive 42
        let val_pos = RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
            DynamicIntegerBackingRef::Execution(DynamicIntegerBackingId(1)),
        ));
        let obs_pos = observe_runtime_value(val_pos, &program, &store);
        match obs_pos {
            Value::Dynamic(InterchangeDynamicValue::Integer(dyn_int)) => {
                assert!(!dyn_int.negative);
                assert!(matches!(dyn_int.magnitude, Cow::Owned(_)));
                assert_eq!(&*dyn_int.magnitude, &[42]);
            }
            _ => panic!("expected dynamic integer"),
        }

        // 2: negative 42
        let val_neg = RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
            DynamicIntegerBackingRef::Execution(DynamicIntegerBackingId(2)),
        ));
        let obs_neg = observe_runtime_value(val_neg, &program, &store);
        match obs_neg {
            Value::Dynamic(InterchangeDynamicValue::Integer(dyn_int)) => {
                assert!(dyn_int.negative);
                assert!(matches!(dyn_int.magnitude, Cow::Owned(_)));
                assert_eq!(&*dyn_int.magnitude, &[42]);
            }
            _ => panic!("expected dynamic integer"),
        }

        // 3: > u128
        let val_large = RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
            DynamicIntegerBackingRef::Execution(DynamicIntegerBackingId(3)),
        ));
        let obs_large = observe_runtime_value(val_large, &program, &store);
        match obs_large {
            Value::Dynamic(InterchangeDynamicValue::Integer(dyn_int)) => {
                assert!(!dyn_int.negative);
                assert!(matches!(dyn_int.magnitude, Cow::Owned(_)));
                let mut expected_mag = vec![0u8; 17];
                expected_mag[0] = 1;
                assert_eq!(&*dyn_int.magnitude, expected_mag.as_slice());
            }
            _ => panic!("expected dynamic integer"),
        }
    }

    #[test]
    fn dynamic_floats() {
        let program = empty_program();
        let store = empty_store();

        let val_f32 = RuntimeValue::Dynamic(RuntimeDynamicValue::Float32(1.25));
        assert_eq!(
            observe_runtime_value(val_f32, &program, &store),
            Value::Dynamic(InterchangeDynamicValue::Float32(1.25))
        );

        let val_f64 = RuntimeValue::Dynamic(RuntimeDynamicValue::Float64(9.875));
        assert_eq!(
            observe_runtime_value(val_f64, &program, &store),
            Value::Dynamic(InterchangeDynamicValue::Float64(9.875))
        );
    }

    #[test]
    fn struct_observation() {
        let mut program = empty_program();
        program
            .constants
            .push(Constant::Dynamic(DynamicConstant::Integer {
                negative: false,
                magnitude: vec![99],
            }));
        let mut store = empty_store();
        store
            .strings
            .push("exec struct str".to_string().into_boxed_str());
        store.structs.push(StructBacking {
            fields: vec![
                RuntimeValue::Int32(100),
                RuntimeValue::String(StringBackingRef::Execution(StringBackingId(0))),
                RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                    DynamicIntegerBackingRef::Compiled(ConstantId(0)),
                )),
            ]
            .into_boxed_slice(),
        });

        let val = RuntimeValue::Struct(StructBackingId(0));
        let observed = observe_runtime_value(val, &program, &store);
        match observed {
            Value::Struct(fields) => {
                assert_eq!(fields.len(), 3);
                assert_eq!(fields[0], Value::Int32(100));

                match fields[1] {
                    Value::String(s) => {
                        assert_eq!(s, "exec struct str");
                        let orig = &store.strings[0];
                        assert_eq!(s.as_ptr(), orig.as_ptr());
                    }
                    _ => panic!("expected Value::String"),
                }

                match &fields[2] {
                    Value::Dynamic(InterchangeDynamicValue::Integer(dyn_int)) => {
                        assert!(!dyn_int.negative);
                        assert!(matches!(dyn_int.magnitude, Cow::Borrowed(_)));
                        assert_eq!(&*dyn_int.magnitude, &[99]);
                    }
                    _ => panic!("expected dynamic integer"),
                }
            }
            _ => panic!("expected Value::Struct"),
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
        let observed = observe_runtime_value(val, &program, &store);
        match observed {
            Value::Struct(fields) => {
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0], Value::Int32(42));
                match &fields[1] {
                    Value::Struct(inner_fields) => {
                        assert_eq!(inner_fields.len(), 1);
                        match inner_fields[0] {
                            Value::String(s) => {
                                assert_eq!(s, "nested string");
                                let orig = &store.strings[0];
                                assert_eq!(s.as_ptr(), orig.as_ptr());
                            }
                            _ => panic!("expected Value::String"),
                        }
                    }
                    _ => panic!("expected nested Value::Struct"),
                }
            }
            _ => panic!("expected Value::Struct"),
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
        let observed = observe_runtime_value(val, &program, &store);
        assert_eq!(
            observed,
            Value::Enum {
                variant: 5,
                payload: EnumPayload::Simple,
            }
        );
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
        let observed = observe_runtime_value(val, &program, &store);
        match observed {
            Value::Enum { variant, payload } => {
                assert_eq!(variant, 1);
                match payload {
                    EnumPayload::Associated(inner) => match *inner {
                        Value::String(s) => {
                            assert_eq!(s, "associated msg");
                            let orig = &store.strings[0];
                            assert_eq!(s.as_ptr(), orig.as_ptr());
                        }
                        _ => panic!("expected Value::String"),
                    },
                    _ => panic!("expected EnumPayload::Associated"),
                }
            }
            _ => panic!("expected Value::Enum"),
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
        let obs_non_empty = observe_runtime_value(val_non_empty, &program, &store);
        match obs_non_empty {
            Value::Enum { variant, payload } => {
                assert_eq!(variant, 2);
                match payload {
                    EnumPayload::Structured { fields } => {
                        assert_eq!(fields.len(), 2);
                        assert_eq!(fields[0], Value::Int32(1));
                        match fields[1] {
                            Value::String(s) => {
                                assert_eq!(s, "payload");
                                let orig = &store.strings[0];
                                assert_eq!(s.as_ptr(), orig.as_ptr());
                            }
                            _ => panic!("expected Value::String"),
                        }
                    }
                    _ => panic!("expected EnumPayload::Structured"),
                }
            }
            _ => panic!("expected Value::Enum"),
        }

        let val_empty = RuntimeValue::Enum(EnumBackingId(1));
        assert_eq!(
            observe_runtime_value(val_empty, &program, &store),
            Value::Enum {
                variant: 3,
                payload: EnumPayload::Structured {
                    fields: vec![].into_boxed_slice(),
                },
            }
        );
    }

    #[test]
    fn deep_composite_tree() {
        let mut program = empty_program();
        program
            .constants
            .push(Constant::Dynamic(DynamicConstant::Integer {
                negative: false,
                magnitude: vec![12],
            }));
        let mut store = empty_store();
        store
            .strings
            .push("deep leaf string".to_string().into_boxed_str());
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
                        DynamicIntegerBackingRef::Compiled(ConstantId(0)),
                    )),
                    RuntimeValue::Dynamic(RuntimeDynamicValue::Integer(
                        DynamicIntegerBackingRef::Execution(DynamicIntegerBackingId(0)),
                    )),
                ]
                .into_boxed_slice(),
            },
        });

        let val = RuntimeValue::Enum(EnumBackingId(0));
        let observed = observe_runtime_value(val, &program, &store);
        match observed {
            Value::Enum { variant, payload } => {
                assert_eq!(variant, 0);
                match payload {
                    EnumPayload::Structured { fields } => {
                        assert_eq!(fields.len(), 3);

                        // Struct field with Execution String
                        match &fields[0] {
                            Value::Struct(st_fields) => {
                                assert_eq!(st_fields.len(), 1);
                                match st_fields[0] {
                                    Value::String(s) => {
                                        assert_eq!(s, "deep leaf string");
                                        let orig = &store.strings[0];
                                        assert_eq!(s.as_ptr(), orig.as_ptr());
                                    }
                                    _ => panic!("expected Value::String"),
                                }
                            }
                            _ => panic!("expected Value::Struct"),
                        }

                        // Compiled Dynamic Integer: Cow::Borrowed
                        match &fields[1] {
                            Value::Dynamic(InterchangeDynamicValue::Integer(dyn_int)) => {
                                assert!(!dyn_int.negative);
                                assert!(matches!(dyn_int.magnitude, Cow::Borrowed(_)));
                                assert_eq!(&*dyn_int.magnitude, &[12]);
                            }
                            _ => panic!("expected compiled dynamic integer"),
                        }

                        // Execution Dynamic Integer: Cow::Owned
                        match &fields[2] {
                            Value::Dynamic(InterchangeDynamicValue::Integer(dyn_int)) => {
                                assert!(dyn_int.negative);
                                assert!(matches!(dyn_int.magnitude, Cow::Owned(_)));
                                assert_eq!(&*dyn_int.magnitude, &[99]);
                            }
                            _ => panic!("expected execution dynamic integer"),
                        }
                    }
                    _ => panic!("expected EnumPayload::Structured"),
                }
            }
            _ => panic!("expected Value::Enum"),
        }
    }

    #[test]
    #[should_panic(expected = "StringBackingId must reference execution backing store")]
    fn dangling_execution_string_backing_id_is_invariant_violation() {
        let program = empty_program();
        let store = empty_store();
        let val = RuntimeValue::String(StringBackingRef::Execution(StringBackingId(99)));
        observe_runtime_value(val, &program, &store);
    }

    #[test]
    #[should_panic(expected = "Expected Constant::String at constant id")]
    fn wrong_constant_family_for_compiled_string_is_invariant_violation() {
        let mut program = empty_program();
        program.constants.push(Constant::Int32(42));
        let store = empty_store();
        let val = RuntimeValue::String(StringBackingRef::Compiled(ConstantId(0)));
        observe_runtime_value(val, &program, &store);
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
        observe_runtime_value(val, &program, &store);
    }

    #[test]
    #[should_panic(expected = "StructBackingId must reference execution backing store")]
    fn dangling_struct_backing_id_is_invariant_violation() {
        let program = empty_program();
        let store = empty_store();
        let val = RuntimeValue::Struct(StructBackingId(99));
        observe_runtime_value(val, &program, &store);
    }

    #[test]
    #[should_panic(expected = "EnumBackingId must reference execution backing store")]
    fn dangling_enum_backing_id_is_invariant_violation() {
        let program = empty_program();
        let store = empty_store();
        let val = RuntimeValue::Enum(EnumBackingId(99));
        observe_runtime_value(val, &program, &store);
    }
}
