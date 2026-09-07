use alloc::boxed::Box;
use alloc::vec::Vec;

use evo_values::OwnedDynamicInteger;

use crate::data::compiled::identities::VariantDiscriminant;
use crate::data::vm::values::RuntimeValue;

pub(crate) struct ExecutionBackingStore {
    pub(crate) strings: Vec<Box<str>>,
    pub(crate) dynamic_integers: Vec<DynamicIntegerBacking>,
    pub(crate) structs: Vec<StructBacking>,
    pub(crate) enums: Vec<EnumBacking>,
}

pub(crate) struct DynamicIntegerBacking {
    pub(crate) value: OwnedDynamicInteger,
}

pub(crate) struct StructBacking {
    pub(crate) fields: Box<[RuntimeValue]>,
}

pub(crate) struct EnumBacking {
    pub(crate) variant: VariantDiscriminant,
    pub(crate) payload: RuntimeEnumPayload,
}

pub(crate) enum RuntimeEnumPayload {
    Simple,
    Associated(RuntimeValue),
    Structured { fields: Box<[RuntimeValue]> },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::vm::values::StructBackingId;
    use alloc::string::ToString;

    #[test]
    fn dynamic_integer_backing_greater_than_u128_and_negative() {
        // Arbitrarily large positive magnitude (greater than u128: 17 bytes)
        let mut large_pos_bytes = [0u8; 17];
        large_pos_bytes[0] = 1;
        let pos_backing = DynamicIntegerBacking {
            value: OwnedDynamicInteger::from_parts(false, Box::from(large_pos_bytes.as_slice())),
        };
        assert!(!pos_backing.value.negative());
        assert_eq!(pos_backing.value.magnitude(), &large_pos_bytes);

        // Arbitrarily large negative magnitude (e.g. 20 bytes)
        let large_neg_bytes = [0xffu8; 20];
        let neg_backing = DynamicIntegerBacking {
            value: OwnedDynamicInteger::from_parts(true, Box::from(large_neg_bytes.as_slice())),
        };
        assert!(neg_backing.value.negative());
        assert_eq!(neg_backing.value.magnitude(), &large_neg_bytes);
    }

    #[test]
    fn execution_backing_store_four_typed_stores() {
        let store = ExecutionBackingStore {
            strings: alloc::vec!["hello world".to_string().into_boxed_str()],
            dynamic_integers: alloc::vec![DynamicIntegerBacking {
                value: OwnedDynamicInteger::from_parts(false, Box::from([100u8].as_slice())),
            }],
            structs: alloc::vec![StructBacking {
                fields: alloc::vec![RuntimeValue::Int32(1)].into_boxed_slice(),
            }],
            enums: alloc::vec![EnumBacking {
                variant: VariantDiscriminant(0),
                payload: RuntimeEnumPayload::Simple,
            }],
        };

        assert_eq!(store.strings.len(), 1);
        assert_eq!(&*store.strings[0], "hello world");
        assert_eq!(store.dynamic_integers.len(), 1);
        assert!(!store.dynamic_integers[0].value.negative());
        assert_eq!(store.dynamic_integers[0].value.magnitude(), &[100u8]);
        assert_eq!(store.structs.len(), 1);
        assert_eq!(store.enums.len(), 1);
    }

    #[test]
    fn struct_backing_fields_positional_order() {
        let sb = StructBacking {
            fields: alloc::vec![
                RuntimeValue::Int32(42),
                RuntimeValue::Boolean(true),
                RuntimeValue::Float64(3.75),
            ]
            .into_boxed_slice(),
        };

        assert_eq!(sb.fields.len(), 3);
        match sb.fields[0] {
            RuntimeValue::Int32(v) => assert_eq!(v, 42),
            _ => panic!("expected Int32 at index 0"),
        }
        match sb.fields[1] {
            RuntimeValue::Boolean(b) => assert!(b),
            _ => panic!("expected Boolean at index 1"),
        }
        match sb.fields[2] {
            RuntimeValue::Float64(v) => assert!((v - 3.75).abs() < 1e-10),
            _ => panic!("expected Float64 at index 2"),
        }
    }

    #[test]
    fn runtime_enum_payload_3_variants() {
        let p_simple = RuntimeEnumPayload::Simple;
        match p_simple {
            RuntimeEnumPayload::Simple => {}
            _ => panic!("expected Simple"),
        }

        let p_assoc = RuntimeEnumPayload::Associated(RuntimeValue::Int32(99));
        match p_assoc {
            RuntimeEnumPayload::Associated(RuntimeValue::Int32(v)) => assert_eq!(v, 99),
            _ => panic!("expected Associated Int32"),
        }

        let p_struct = RuntimeEnumPayload::Structured {
            fields: alloc::vec![RuntimeValue::Int32(1), RuntimeValue::Boolean(false)]
                .into_boxed_slice(),
        };
        match p_struct {
            RuntimeEnumPayload::Structured { fields } => {
                assert_eq!(fields.len(), 2);
                match fields[0] {
                    RuntimeValue::Int32(v) => assert_eq!(v, 1),
                    _ => panic!("expected Int32 at field 0"),
                }
                match fields[1] {
                    RuntimeValue::Boolean(b) => assert!(!b),
                    _ => panic!("expected Boolean at field 1"),
                }
            }
            _ => panic!("expected Structured"),
        }
    }

    #[test]
    fn immutable_dag_sharing_representation() {
        // Leaf struct backing at index 0
        let leaf = StructBacking {
            fields: alloc::vec![RuntimeValue::Int32(100)].into_boxed_slice(),
        };

        // Parent struct backing at index 1 sharing two references to StructBackingId(0)
        let parent = StructBacking {
            fields: alloc::vec![
                RuntimeValue::Struct(StructBackingId(0)),
                RuntimeValue::Struct(StructBackingId(0)),
            ]
            .into_boxed_slice(),
        };

        let store = ExecutionBackingStore {
            strings: alloc::vec![],
            dynamic_integers: alloc::vec![],
            structs: alloc::vec![leaf, parent],
            enums: alloc::vec![],
        };

        assert_eq!(store.structs.len(), 2);
        match store.structs[1].fields[0] {
            RuntimeValue::Struct(id) => assert_eq!(id.0, 0),
            _ => panic!("expected StructBackingId(0) at field 0"),
        }
        match store.structs[1].fields[1] {
            RuntimeValue::Struct(id) => assert_eq!(id.0, 0),
            _ => panic!("expected StructBackingId(0) at field 1"),
        }
    }
}
