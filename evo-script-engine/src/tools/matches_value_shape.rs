use crate::data::compiled::boundary::{CompiledEnumValueShape, CompiledValueShape};
use crate::data::compiled::identities::CompiledValueShapeId;
use evo_values::{DynamicValue, EnumPayload, Value};

pub type MatchesValueShape =
    for<'value> fn(&Value<'value>, CompiledValueShapeId, &[CompiledValueShape]) -> bool;

pub fn matches_value_shape<'value>(
    value: &Value<'value>,
    shape_id: CompiledValueShapeId,
    shapes: &[CompiledValueShape],
) -> bool {
    let shape = shapes
        .get(shape_id.0)
        .expect("CompiledValueShapeId must reference the provided shape table");

    match (value, shape) {
        (Value::Boolean(_), CompiledValueShape::Boolean) => true,

        (Value::Int8(_), CompiledValueShape::Int8) => true,
        (Value::Int16(_), CompiledValueShape::Int16) => true,
        (Value::Int32(_), CompiledValueShape::Int32) => true,
        (Value::Int64(_), CompiledValueShape::Int64) => true,
        (Value::Int128(_), CompiledValueShape::Int128) => true,

        (Value::Uint8(_), CompiledValueShape::Uint8) => true,
        (Value::Uint16(_), CompiledValueShape::Uint16) => true,
        (Value::Uint32(_), CompiledValueShape::Uint32) => true,
        (Value::Uint64(_), CompiledValueShape::Uint64) => true,
        (Value::Uint128(_), CompiledValueShape::Uint128) => true,

        (Value::Float32(_), CompiledValueShape::Float32) => true,
        (Value::Float64(_), CompiledValueShape::Float64) => true,

        (Value::String(_), CompiledValueShape::String) => true,

        (Value::Dynamic(dyn_val), CompiledValueShape::Dynamic) => match dyn_val {
            DynamicValue::Integer(_) | DynamicValue::Float32(_) | DynamicValue::Float64(_) => true,
        },

        (
            Value::Struct(val_fields),
            CompiledValueShape::Struct {
                fields: shape_fields,
            },
        ) => {
            if val_fields.len() != shape_fields.len() {
                return false;
            }
            for (vf, sf_id) in val_fields.iter().zip(shape_fields.iter()) {
                if !matches_value_shape(vf, CompiledValueShapeId(sf_id.0), shapes) {
                    return false;
                }
            }
            true
        }

        (
            Value::Enum { variant, payload },
            CompiledValueShape::Enum {
                variants: shape_variants,
            },
        ) => {
            let shape_variant = match shape_variants.get(*variant) {
                Some(v) => v,
                None => return false,
            };

            match (payload, shape_variant) {
                (EnumPayload::Simple, CompiledEnumValueShape::Simple) => true,
                (
                    EnumPayload::Associated(associated_val),
                    CompiledEnumValueShape::Associated(target_shape_id),
                ) => matches_value_shape(
                    associated_val,
                    CompiledValueShapeId(target_shape_id.0),
                    shapes,
                ),
                (
                    EnumPayload::Structured { fields: val_fields },
                    CompiledEnumValueShape::Structured {
                        fields: shape_fields,
                    },
                ) => {
                    if val_fields.len() != shape_fields.len() {
                        return false;
                    }
                    for (vf, sf_id) in val_fields.iter().zip(shape_fields.iter()) {
                        if !matches_value_shape(vf, CompiledValueShapeId(sf_id.0), shapes) {
                            return false;
                        }
                    }
                    true
                }
                _ => false,
            }
        }

        _ => false,
    }
}

pub const MATCHES_VALUE_SHAPE: MatchesValueShape = matches_value_shape;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::borrow::Cow;
    use alloc::boxed::Box;
    use alloc::vec;
    use evo_values::DynamicIntegerValue;

    #[test]
    fn typed_binding() {
        let implementation: MatchesValueShape = matches_value_shape;
        let binding: MatchesValueShape = MATCHES_VALUE_SHAPE;
        assert_eq!(implementation as usize, binding as usize);
    }

    #[test]
    fn all_scalar_families_positive() {
        let shapes = [
            CompiledValueShape::Boolean,
            CompiledValueShape::Int8,
            CompiledValueShape::Int16,
            CompiledValueShape::Int32,
            CompiledValueShape::Int64,
            CompiledValueShape::Int128,
            CompiledValueShape::Uint8,
            CompiledValueShape::Uint16,
            CompiledValueShape::Uint32,
            CompiledValueShape::Uint64,
            CompiledValueShape::Uint128,
            CompiledValueShape::Float32,
            CompiledValueShape::Float64,
            CompiledValueShape::String,
            CompiledValueShape::Dynamic,
        ];

        let values = [
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
            Value::Float32(1.5),
            Value::Float64(2.5),
            Value::String("hello"),
            Value::Dynamic(DynamicValue::Float64(42.0)),
        ];

        for (i, val) in values.iter().enumerate() {
            assert!(matches_value_shape(val, CompiledValueShapeId(i), &shapes));
        }
    }

    #[test]
    fn numeric_mismatches() {
        let shapes = [
            CompiledValueShape::Int64,
            CompiledValueShape::Int32,
            CompiledValueShape::Float64,
            CompiledValueShape::Uint32,
        ];

        assert!(!matches_value_shape(
            &Value::Int32(42),
            CompiledValueShapeId(0),
            &shapes
        ));
        assert!(!matches_value_shape(
            &Value::Int8(42),
            CompiledValueShapeId(1),
            &shapes
        ));
        assert!(!matches_value_shape(
            &Value::Float32(1.0),
            CompiledValueShapeId(2),
            &shapes
        ));
        assert!(!matches_value_shape(
            &Value::Int32(42),
            CompiledValueShapeId(3),
            &shapes
        ));
        assert!(!matches_value_shape(
            &Value::String("42"),
            CompiledValueShapeId(1),
            &shapes
        ));
    }

    #[test]
    fn dynamic_exactness() {
        let shapes = [
            CompiledValueShape::Dynamic,
            CompiledValueShape::Int32,
            CompiledValueShape::Float64,
        ];

        let dyn_int = Value::Dynamic(DynamicValue::Integer(DynamicIntegerValue {
            negative: false,
            magnitude: Cow::Borrowed(&[10]),
        }));
        let dyn_f32 = Value::Dynamic(DynamicValue::Float32(1.5));
        let dyn_f64 = Value::Dynamic(DynamicValue::Float64(2.5));

        assert!(matches_value_shape(
            &dyn_int,
            CompiledValueShapeId(0),
            &shapes
        ));
        assert!(matches_value_shape(
            &dyn_f32,
            CompiledValueShapeId(0),
            &shapes
        ));
        assert!(matches_value_shape(
            &dyn_f64,
            CompiledValueShapeId(0),
            &shapes
        ));

        assert!(!matches_value_shape(
            &Value::Int32(10),
            CompiledValueShapeId(0),
            &shapes
        ));
        assert!(!matches_value_shape(
            &Value::Float64(2.5),
            CompiledValueShapeId(0),
            &shapes
        ));
        assert!(!matches_value_shape(
            &dyn_int,
            CompiledValueShapeId(1),
            &shapes
        ));
        assert!(!matches_value_shape(
            &dyn_f64,
            CompiledValueShapeId(2),
            &shapes
        ));
    }

    #[test]
    fn struct_exact_success() {
        let shapes = [
            CompiledValueShape::Int32,  // 0
            CompiledValueShape::String, // 1
            CompiledValueShape::Struct {
                // 2
                fields: vec![CompiledValueShapeId(0), CompiledValueShapeId(1)],
            },
        ];

        let val = Value::Struct(Box::new([Value::Int32(42), Value::String("Alice")]));
        assert!(matches_value_shape(&val, CompiledValueShapeId(2), &shapes));
    }

    #[test]
    fn struct_cardinality_mismatch() {
        let shapes = [
            CompiledValueShape::Int32,  // 0
            CompiledValueShape::String, // 1
            CompiledValueShape::Struct {
                // 2
                fields: vec![CompiledValueShapeId(0), CompiledValueShapeId(1)],
            },
        ];

        let val_few = Value::Struct(Box::new([Value::Int32(42)]));
        let val_many = Value::Struct(Box::new([
            Value::Int32(42),
            Value::String("Alice"),
            Value::Boolean(true),
        ]));

        assert!(!matches_value_shape(
            &val_few,
            CompiledValueShapeId(2),
            &shapes
        ));
        assert!(!matches_value_shape(
            &val_many,
            CompiledValueShapeId(2),
            &shapes
        ));
    }

    #[test]
    fn struct_nested_type_mismatch() {
        let shapes = [
            CompiledValueShape::Int32,  // 0
            CompiledValueShape::String, // 1
            CompiledValueShape::Struct {
                // 2
                fields: vec![CompiledValueShapeId(0), CompiledValueShapeId(1)],
            },
        ];

        let val_wrong_type = Value::Struct(Box::new([Value::Int32(42), Value::Int32(100)]));
        assert!(!matches_value_shape(
            &val_wrong_type,
            CompiledValueShapeId(2),
            &shapes
        ));
    }

    #[test]
    fn nested_struct() {
        let shapes = [
            CompiledValueShape::Int32,   // 0
            CompiledValueShape::String,  // 1
            CompiledValueShape::Boolean, // 2
            CompiledValueShape::Struct {
                // 3: Inner [String, Boolean]
                fields: vec![CompiledValueShapeId(1), CompiledValueShapeId(2)],
            },
            CompiledValueShape::Struct {
                // 4: Outer [Int32, Inner]
                fields: vec![CompiledValueShapeId(0), CompiledValueShapeId(3)],
            },
        ];

        let val = Value::Struct(Box::new([
            Value::Int32(42),
            Value::Struct(Box::new([Value::String("Bob"), Value::Boolean(false)])),
        ]));

        assert!(matches_value_shape(&val, CompiledValueShapeId(4), &shapes));
    }

    #[test]
    fn enum_simple_and_invalid_ordinal() {
        let shapes = [CompiledValueShape::Enum {
            variants: vec![
                CompiledEnumValueShape::Simple,
                CompiledEnumValueShape::Simple,
            ],
        }];

        let val_v0 = Value::Enum {
            variant: 0,
            payload: EnumPayload::Simple,
        };
        let val_v1 = Value::Enum {
            variant: 1,
            payload: EnumPayload::Simple,
        };
        let val_out_of_bounds = Value::Enum {
            variant: 2,
            payload: EnumPayload::Simple,
        };
        let val_wrong_payload = Value::Enum {
            variant: 0,
            payload: EnumPayload::Associated(Box::new(Value::Int32(10))),
        };

        assert!(matches_value_shape(
            &val_v0,
            CompiledValueShapeId(0),
            &shapes
        ));
        assert!(matches_value_shape(
            &val_v1,
            CompiledValueShapeId(0),
            &shapes
        ));
        assert!(!matches_value_shape(
            &val_out_of_bounds,
            CompiledValueShapeId(0),
            &shapes
        ));
        assert!(!matches_value_shape(
            &val_wrong_payload,
            CompiledValueShapeId(0),
            &shapes
        ));
    }

    #[test]
    fn enum_associated() {
        let shapes = [
            CompiledValueShape::Int32, // 0
            CompiledValueShape::Enum {
                // 1
                variants: vec![
                    CompiledEnumValueShape::Simple,
                    CompiledEnumValueShape::Associated(CompiledValueShapeId(0)),
                ],
            },
        ];

        let val_correct = Value::Enum {
            variant: 1,
            payload: EnumPayload::Associated(Box::new(Value::Int32(100))),
        };
        let val_wrong_type = Value::Enum {
            variant: 1,
            payload: EnumPayload::Associated(Box::new(Value::String("wrong"))),
        };
        let val_wrong_shape_kind = Value::Enum {
            variant: 1,
            payload: EnumPayload::Simple,
        };

        assert!(matches_value_shape(
            &val_correct,
            CompiledValueShapeId(1),
            &shapes
        ));
        assert!(!matches_value_shape(
            &val_wrong_type,
            CompiledValueShapeId(1),
            &shapes
        ));
        assert!(!matches_value_shape(
            &val_wrong_shape_kind,
            CompiledValueShapeId(1),
            &shapes
        ));
    }

    #[test]
    fn enum_structured() {
        let shapes = [
            CompiledValueShape::Int32,  // 0
            CompiledValueShape::String, // 1
            CompiledValueShape::Enum {
                // 2
                variants: vec![CompiledEnumValueShape::Structured {
                    fields: vec![CompiledValueShapeId(0), CompiledValueShapeId(1)],
                }],
            },
        ];

        let val_correct = Value::Enum {
            variant: 0,
            payload: EnumPayload::Structured {
                fields: Box::new([Value::Int32(200), Value::String("OK")]),
            },
        };
        let val_wrong_cardinality = Value::Enum {
            variant: 0,
            payload: EnumPayload::Structured {
                fields: Box::new([Value::Int32(200)]),
            },
        };
        let val_wrong_nested = Value::Enum {
            variant: 0,
            payload: EnumPayload::Structured {
                fields: Box::new([Value::Int32(200), Value::Int32(300)]),
            },
        };
        let val_wrong_family = Value::Enum {
            variant: 0,
            payload: EnumPayload::Simple,
        };

        assert!(matches_value_shape(
            &val_correct,
            CompiledValueShapeId(2),
            &shapes
        ));
        assert!(!matches_value_shape(
            &val_wrong_cardinality,
            CompiledValueShapeId(2),
            &shapes
        ));
        assert!(!matches_value_shape(
            &val_wrong_nested,
            CompiledValueShapeId(2),
            &shapes
        ));
        assert!(!matches_value_shape(
            &val_wrong_family,
            CompiledValueShapeId(2),
            &shapes
        ));
    }

    #[test]
    fn deep_composite_tree() {
        let shapes = [
            CompiledValueShape::String,  // 0
            CompiledValueShape::Dynamic, // 1
            CompiledValueShape::Struct {
                // 2: Struct [String]
                fields: vec![CompiledValueShapeId(0)],
            },
            CompiledValueShape::Enum {
                // 3: Enum [ Structured { Struct[String], Dynamic } ]
                variants: vec![CompiledEnumValueShape::Structured {
                    fields: vec![CompiledValueShapeId(2), CompiledValueShapeId(1)],
                }],
            },
        ];

        let val = Value::Enum {
            variant: 0,
            payload: EnumPayload::Structured {
                fields: Box::new([
                    Value::Struct(Box::new([Value::String("nested")])),
                    Value::Dynamic(DynamicValue::Float64(99.9)),
                ]),
            },
        };

        assert!(matches_value_shape(&val, CompiledValueShapeId(3), &shapes));
    }

    #[test]
    #[should_panic(expected = "CompiledValueShapeId must reference the provided shape table")]
    fn dangling_root_shape_id_is_invariant_violation() {
        let shapes = [CompiledValueShape::Int32];
        let val = Value::Int32(42);
        matches_value_shape(&val, CompiledValueShapeId(99), &shapes);
    }

    #[test]
    #[should_panic(expected = "CompiledValueShapeId must reference the provided shape table")]
    fn dangling_nested_shape_id_is_invariant_violation() {
        let shapes = [CompiledValueShape::Struct {
            fields: vec![CompiledValueShapeId(99)],
        }];
        let val = Value::Struct(Box::new([Value::Int32(42)]));
        matches_value_shape(&val, CompiledValueShapeId(0), &shapes);
    }
}
