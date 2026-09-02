use crate::data::compiled::boundary::{CompiledEnumValueShape, CompiledValueShape};
use crate::data::compiled::identities::CompiledValueShapeId;
use evo_values::{OwnedDynamicValue, OwnedEnumPayload, OwnedValue};

pub type MatchesOwnedValueShape =
    fn(&OwnedValue, CompiledValueShapeId, &[CompiledValueShape]) -> bool;

pub fn matches_owned_value_shape(
    value: &OwnedValue,
    expected_shape: CompiledValueShapeId,
    value_shapes: &[CompiledValueShape],
) -> bool {
    let shape = value_shapes
        .get(expected_shape.0)
        .expect("CompiledValueShapeId must reference compiled value shape");

    match (value, shape) {
        (OwnedValue::Boolean(_), CompiledValueShape::Boolean) => true,

        (OwnedValue::Int8(_), CompiledValueShape::Int8) => true,
        (OwnedValue::Int16(_), CompiledValueShape::Int16) => true,
        (OwnedValue::Int32(_), CompiledValueShape::Int32) => true,
        (OwnedValue::Int64(_), CompiledValueShape::Int64) => true,
        (OwnedValue::Int128(_), CompiledValueShape::Int128) => true,

        (OwnedValue::Uint8(_), CompiledValueShape::Uint8) => true,
        (OwnedValue::Uint16(_), CompiledValueShape::Uint16) => true,
        (OwnedValue::Uint32(_), CompiledValueShape::Uint32) => true,
        (OwnedValue::Uint64(_), CompiledValueShape::Uint64) => true,
        (OwnedValue::Uint128(_), CompiledValueShape::Uint128) => true,

        (OwnedValue::Float32(_), CompiledValueShape::Float32) => true,
        (OwnedValue::Float64(_), CompiledValueShape::Float64) => true,

        (OwnedValue::String(_), CompiledValueShape::String) => true,

        (OwnedValue::Dynamic(dyn_val), CompiledValueShape::Dynamic) => match dyn_val {
            OwnedDynamicValue::Integer(_)
            | OwnedDynamicValue::Float32(_)
            | OwnedDynamicValue::Float64(_) => true,
        },

        (
            OwnedValue::Struct(actual_fields),
            CompiledValueShape::Struct {
                fields: expected_fields,
            },
        ) => {
            if actual_fields.len() != expected_fields.len() {
                return false;
            }
            for (vf, sf_id) in actual_fields.iter().zip(expected_fields.iter()) {
                if !matches_owned_value_shape(vf, CompiledValueShapeId(sf_id.0), value_shapes) {
                    return false;
                }
            }
            true
        }

        (
            OwnedValue::Enum { variant, payload },
            CompiledValueShape::Enum {
                variants: shape_variants,
            },
        ) => {
            let shape_variant = match shape_variants.get(*variant) {
                Some(v) => v,
                None => return false,
            };

            match (payload, shape_variant) {
                (OwnedEnumPayload::Simple, CompiledEnumValueShape::Simple) => true,
                (
                    OwnedEnumPayload::Associated(associated_val),
                    CompiledEnumValueShape::Associated(target_shape_id),
                ) => matches_owned_value_shape(
                    associated_val,
                    CompiledValueShapeId(target_shape_id.0),
                    value_shapes,
                ),
                (
                    OwnedEnumPayload::Structured {
                        fields: actual_fields,
                    },
                    CompiledEnumValueShape::Structured {
                        fields: expected_fields,
                    },
                ) => {
                    if actual_fields.len() != expected_fields.len() {
                        return false;
                    }
                    for (vf, sf_id) in actual_fields.iter().zip(expected_fields.iter()) {
                        if !matches_owned_value_shape(
                            vf,
                            CompiledValueShapeId(sf_id.0),
                            value_shapes,
                        ) {
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

pub const MATCHES_OWNED_VALUE_SHAPE: MatchesOwnedValueShape = matches_owned_value_shape;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::boxed::Box;
    use alloc::string::ToString;
    use alloc::vec;
    use evo_values::OwnedDynamicInteger;

    fn sample_shapes() -> [CompiledValueShape; 15] {
        [
            CompiledValueShape::Boolean, // 0
            CompiledValueShape::Int8,    // 1
            CompiledValueShape::Int16,   // 2
            CompiledValueShape::Int32,   // 3
            CompiledValueShape::Int64,   // 4
            CompiledValueShape::Int128,  // 5
            CompiledValueShape::Uint8,   // 6
            CompiledValueShape::Uint16,  // 7
            CompiledValueShape::Uint32,  // 8
            CompiledValueShape::Uint64,  // 9
            CompiledValueShape::Uint128, // 10
            CompiledValueShape::Float32, // 11
            CompiledValueShape::Float64, // 12
            CompiledValueShape::String,  // 13
            CompiledValueShape::Dynamic, // 14
        ]
    }

    #[test]
    fn typed_binding() {
        let implementation: MatchesOwnedValueShape = matches_owned_value_shape;
        let binding: MatchesOwnedValueShape = MATCHES_OWNED_VALUE_SHAPE;
        assert_eq!(implementation as usize, binding as usize);
    }

    #[test]
    fn exact_fixed_families() {
        let shapes = sample_shapes();

        assert!(matches_owned_value_shape(
            &OwnedValue::Boolean(true),
            CompiledValueShapeId(0),
            &shapes
        ));
        assert!(matches_owned_value_shape(
            &OwnedValue::Int8(1),
            CompiledValueShapeId(1),
            &shapes
        ));
        assert!(matches_owned_value_shape(
            &OwnedValue::Int16(1),
            CompiledValueShapeId(2),
            &shapes
        ));
        assert!(matches_owned_value_shape(
            &OwnedValue::Int32(1),
            CompiledValueShapeId(3),
            &shapes
        ));
        assert!(matches_owned_value_shape(
            &OwnedValue::Int64(1),
            CompiledValueShapeId(4),
            &shapes
        ));
        assert!(matches_owned_value_shape(
            &OwnedValue::Int128(1),
            CompiledValueShapeId(5),
            &shapes
        ));
        assert!(matches_owned_value_shape(
            &OwnedValue::Uint8(1),
            CompiledValueShapeId(6),
            &shapes
        ));
        assert!(matches_owned_value_shape(
            &OwnedValue::Uint16(1),
            CompiledValueShapeId(7),
            &shapes
        ));
        assert!(matches_owned_value_shape(
            &OwnedValue::Uint32(1),
            CompiledValueShapeId(8),
            &shapes
        ));
        assert!(matches_owned_value_shape(
            &OwnedValue::Uint64(1),
            CompiledValueShapeId(9),
            &shapes
        ));
        assert!(matches_owned_value_shape(
            &OwnedValue::Uint128(1),
            CompiledValueShapeId(10),
            &shapes
        ));
        assert!(matches_owned_value_shape(
            &OwnedValue::Float32(1.0),
            CompiledValueShapeId(11),
            &shapes
        ));
        assert!(matches_owned_value_shape(
            &OwnedValue::Float64(1.0),
            CompiledValueShapeId(12),
            &shapes
        ));
        assert!(matches_owned_value_shape(
            &OwnedValue::String("hello".to_string().into_boxed_str()),
            CompiledValueShapeId(13),
            &shapes
        ));
    }

    #[test]
    fn numeric_width_and_sign_mismatch() {
        let shapes = sample_shapes();

        assert!(!matches_owned_value_shape(
            &OwnedValue::Int8(1),
            CompiledValueShapeId(2),
            &shapes
        ));
        assert!(!matches_owned_value_shape(
            &OwnedValue::Int32(1),
            CompiledValueShapeId(8),
            &shapes
        ));
        assert!(!matches_owned_value_shape(
            &OwnedValue::Uint64(1),
            CompiledValueShapeId(10),
            &shapes
        ));
        assert!(!matches_owned_value_shape(
            &OwnedValue::Float32(1.0),
            CompiledValueShapeId(12),
            &shapes
        ));
        assert!(!matches_owned_value_shape(
            &OwnedValue::String("s".to_string().into_boxed_str()),
            CompiledValueShapeId(14),
            &shapes
        ));
    }

    #[test]
    fn dynamic_exactness() {
        let shapes = sample_shapes();

        assert!(matches_owned_value_shape(
            &OwnedValue::Dynamic(OwnedDynamicValue::Integer(OwnedDynamicInteger {
                negative: false,
                magnitude: vec![1, 2, 3].into_boxed_slice(),
            })),
            CompiledValueShapeId(14),
            &shapes
        ));
        assert!(matches_owned_value_shape(
            &OwnedValue::Dynamic(OwnedDynamicValue::Float32(1.5)),
            CompiledValueShapeId(14),
            &shapes
        ));
        assert!(matches_owned_value_shape(
            &OwnedValue::Dynamic(OwnedDynamicValue::Float64(2.5)),
            CompiledValueShapeId(14),
            &shapes
        ));

        // Fixed numeric vs dynamic
        assert!(!matches_owned_value_shape(
            &OwnedValue::Int32(10),
            CompiledValueShapeId(14),
            &shapes
        ));
        assert!(!matches_owned_value_shape(
            &OwnedValue::Dynamic(OwnedDynamicValue::Float32(1.5)),
            CompiledValueShapeId(3),
            &shapes
        ));
    }

    #[test]
    fn struct_exact() {
        let shapes = [
            CompiledValueShape::Int32,   // 0
            CompiledValueShape::String,  // 1
            CompiledValueShape::Dynamic, // 2
            CompiledValueShape::Struct {
                // 3
                fields: vec![
                    CompiledValueShapeId(0),
                    CompiledValueShapeId(1),
                    CompiledValueShapeId(2),
                ],
            },
        ];

        let val = OwnedValue::Struct(
            vec![
                OwnedValue::Int32(42),
                OwnedValue::String("text".to_string().into_boxed_str()),
                OwnedValue::Dynamic(OwnedDynamicValue::Float32(1.0)),
            ]
            .into_boxed_slice(),
        );

        assert!(matches_owned_value_shape(
            &val,
            CompiledValueShapeId(3),
            &shapes
        ));
    }

    #[test]
    fn struct_cardinality_mismatch() {
        let shapes = [
            CompiledValueShape::Int32, // 0
            CompiledValueShape::Struct {
                // 1
                fields: vec![CompiledValueShapeId(0), CompiledValueShapeId(0)],
            },
        ];

        let too_few = OwnedValue::Struct(vec![OwnedValue::Int32(1)].into_boxed_slice());
        let too_many = OwnedValue::Struct(
            vec![
                OwnedValue::Int32(1),
                OwnedValue::Int32(2),
                OwnedValue::Int32(3),
            ]
            .into_boxed_slice(),
        );

        assert!(!matches_owned_value_shape(
            &too_few,
            CompiledValueShapeId(1),
            &shapes
        ));
        assert!(!matches_owned_value_shape(
            &too_many,
            CompiledValueShapeId(1),
            &shapes
        ));
    }

    #[test]
    fn struct_positional_mismatch() {
        let shapes = [
            CompiledValueShape::Int32,  // 0
            CompiledValueShape::String, // 1
            CompiledValueShape::Struct {
                // 2
                fields: vec![CompiledValueShapeId(0), CompiledValueShapeId(1)],
            },
        ];

        let wrong_order = OwnedValue::Struct(
            vec![
                OwnedValue::String("wrong".to_string().into_boxed_str()),
                OwnedValue::Int32(42),
            ]
            .into_boxed_slice(),
        );

        assert!(!matches_owned_value_shape(
            &wrong_order,
            CompiledValueShapeId(2),
            &shapes
        ));
    }

    #[test]
    fn nested_struct() {
        let shapes = [
            CompiledValueShape::Int32,   // 0
            CompiledValueShape::String,  // 1
            CompiledValueShape::Dynamic, // 2
            CompiledValueShape::Struct {
                // 3: inner
                fields: vec![CompiledValueShapeId(1), CompiledValueShapeId(2)],
            },
            CompiledValueShape::Struct {
                // 4: outer
                fields: vec![CompiledValueShapeId(0), CompiledValueShapeId(3)],
            },
        ];

        let valid_nested = OwnedValue::Struct(
            vec![
                OwnedValue::Int32(10),
                OwnedValue::Struct(
                    vec![
                        OwnedValue::String("inner".to_string().into_boxed_str()),
                        OwnedValue::Dynamic(OwnedDynamicValue::Float32(1.0)),
                    ]
                    .into_boxed_slice(),
                ),
            ]
            .into_boxed_slice(),
        );

        let invalid_nested = OwnedValue::Struct(
            vec![
                OwnedValue::Int32(10),
                OwnedValue::Struct(
                    vec![
                        OwnedValue::Int32(99), // Mismatch here: expected String
                        OwnedValue::Dynamic(OwnedDynamicValue::Float32(1.0)),
                    ]
                    .into_boxed_slice(),
                ),
            ]
            .into_boxed_slice(),
        );

        assert!(matches_owned_value_shape(
            &valid_nested,
            CompiledValueShapeId(4),
            &shapes
        ));
        assert!(!matches_owned_value_shape(
            &invalid_nested,
            CompiledValueShapeId(4),
            &shapes
        ));
    }

    #[test]
    fn empty_struct() {
        let shapes = [CompiledValueShape::Struct { fields: vec![] }];

        let empty_val = OwnedValue::Struct(vec![].into_boxed_slice());
        assert!(matches_owned_value_shape(
            &empty_val,
            CompiledValueShapeId(0),
            &shapes
        ));
    }

    #[test]
    fn enum_simple() {
        let shapes = [CompiledValueShape::Enum {
            variants: vec![
                CompiledEnumValueShape::Simple,
                CompiledEnumValueShape::Simple,
            ],
        }];

        let val_v0 = OwnedValue::Enum {
            variant: 0,
            payload: OwnedEnumPayload::Simple,
        };
        let val_v1 = OwnedValue::Enum {
            variant: 1,
            payload: OwnedEnumPayload::Simple,
        };

        assert!(matches_owned_value_shape(
            &val_v0,
            CompiledValueShapeId(0),
            &shapes
        ));
        assert!(matches_owned_value_shape(
            &val_v1,
            CompiledValueShapeId(0),
            &shapes
        ));
    }

    #[test]
    fn enum_ordinal_mismatch_and_out_of_range() {
        let shapes = [
            CompiledValueShape::Int32, // 0
            CompiledValueShape::Enum {
                // 1
                variants: vec![
                    CompiledEnumValueShape::Simple,                              // 0
                    CompiledEnumValueShape::Associated(CompiledValueShapeId(0)), // 1
                ],
            },
        ];

        // Variant 0 is Simple, but payload is Associated
        let mismatch_payload = OwnedValue::Enum {
            variant: 0,
            payload: OwnedEnumPayload::Associated(Box::new(OwnedValue::Int32(10))),
        };
        assert!(!matches_owned_value_shape(
            &mismatch_payload,
            CompiledValueShapeId(1),
            &shapes
        ));

        // Variant ordinal out of range (>= variants.len()) -> must return false, NOT panic
        let out_of_range = OwnedValue::Enum {
            variant: 2,
            payload: OwnedEnumPayload::Simple,
        };
        assert!(!matches_owned_value_shape(
            &out_of_range,
            CompiledValueShapeId(1),
            &shapes
        ));
    }

    #[test]
    fn enum_associated() {
        let shapes = [
            CompiledValueShape::Int32,  // 0
            CompiledValueShape::String, // 1
            CompiledValueShape::Enum {
                // 2
                variants: vec![CompiledEnumValueShape::Associated(CompiledValueShapeId(0))],
            },
        ];

        let valid = OwnedValue::Enum {
            variant: 0,
            payload: OwnedEnumPayload::Associated(Box::new(OwnedValue::Int32(123))),
        };
        let invalid_type = OwnedValue::Enum {
            variant: 0,
            payload: OwnedEnumPayload::Associated(Box::new(OwnedValue::String(
                "s".to_string().into_boxed_str(),
            ))),
        };

        assert!(matches_owned_value_shape(
            &valid,
            CompiledValueShapeId(2),
            &shapes
        ));
        assert!(!matches_owned_value_shape(
            &invalid_type,
            CompiledValueShapeId(2),
            &shapes
        ));
    }

    #[test]
    fn associated_wrong_payload_family() {
        let shapes = [
            CompiledValueShape::Int32, // 0
            CompiledValueShape::Enum {
                // 1
                variants: vec![CompiledEnumValueShape::Associated(CompiledValueShapeId(0))],
            },
        ];

        let simple_payload = OwnedValue::Enum {
            variant: 0,
            payload: OwnedEnumPayload::Simple,
        };
        let structured_payload = OwnedValue::Enum {
            variant: 0,
            payload: OwnedEnumPayload::Structured {
                fields: vec![OwnedValue::Int32(10)].into_boxed_slice(),
            },
        };

        assert!(!matches_owned_value_shape(
            &simple_payload,
            CompiledValueShapeId(1),
            &shapes
        ));
        assert!(!matches_owned_value_shape(
            &structured_payload,
            CompiledValueShapeId(1),
            &shapes
        ));
    }

    #[test]
    fn enum_structured() {
        let shapes = [
            CompiledValueShape::String, // 0
            CompiledValueShape::Int32,  // 1
            CompiledValueShape::Enum {
                // 2
                variants: vec![CompiledEnumValueShape::Structured {
                    fields: vec![CompiledValueShapeId(0), CompiledValueShapeId(1)],
                }],
            },
        ];

        let valid = OwnedValue::Enum {
            variant: 0,
            payload: OwnedEnumPayload::Structured {
                fields: vec![
                    OwnedValue::String("field".to_string().into_boxed_str()),
                    OwnedValue::Int32(10),
                ]
                .into_boxed_slice(),
            },
        };
        let wrong_field = OwnedValue::Enum {
            variant: 0,
            payload: OwnedEnumPayload::Structured {
                fields: vec![
                    OwnedValue::Int32(99), // expected String
                    OwnedValue::Int32(10),
                ]
                .into_boxed_slice(),
            },
        };
        let wrong_cardinality = OwnedValue::Enum {
            variant: 0,
            payload: OwnedEnumPayload::Structured {
                fields: vec![OwnedValue::String("field".to_string().into_boxed_str())]
                    .into_boxed_slice(),
            },
        };
        let wrong_order = OwnedValue::Enum {
            variant: 0,
            payload: OwnedEnumPayload::Structured {
                fields: vec![
                    OwnedValue::Int32(10),
                    OwnedValue::String("field".to_string().into_boxed_str()),
                ]
                .into_boxed_slice(),
            },
        };

        assert!(matches_owned_value_shape(
            &valid,
            CompiledValueShapeId(2),
            &shapes
        ));
        assert!(!matches_owned_value_shape(
            &wrong_field,
            CompiledValueShapeId(2),
            &shapes
        ));
        assert!(!matches_owned_value_shape(
            &wrong_cardinality,
            CompiledValueShapeId(2),
            &shapes
        ));
        assert!(!matches_owned_value_shape(
            &wrong_order,
            CompiledValueShapeId(2),
            &shapes
        ));
    }

    #[test]
    fn empty_enum_structured() {
        let shapes = [CompiledValueShape::Enum {
            variants: vec![CompiledEnumValueShape::Structured { fields: vec![] }],
        }];

        let valid_empty_structured = OwnedValue::Enum {
            variant: 0,
            payload: OwnedEnumPayload::Structured {
                fields: vec![].into_boxed_slice(),
            },
        };
        let simple_payload = OwnedValue::Enum {
            variant: 0,
            payload: OwnedEnumPayload::Simple,
        };

        assert!(matches_owned_value_shape(
            &valid_empty_structured,
            CompiledValueShapeId(0),
            &shapes
        ));
        assert!(!matches_owned_value_shape(
            &simple_payload,
            CompiledValueShapeId(0),
            &shapes
        ));
    }

    #[test]
    fn deep_composite() {
        let shapes = [
            CompiledValueShape::String,  // 0
            CompiledValueShape::Dynamic, // 1
            CompiledValueShape::Int64,   // 2
            CompiledValueShape::Struct {
                // 3: Struct [String, Dynamic]
                fields: vec![CompiledValueShapeId(0), CompiledValueShapeId(1)],
            },
            CompiledValueShape::Enum {
                // 4: Enum variant 0: Associated(Int64)
                variants: vec![CompiledEnumValueShape::Associated(CompiledValueShapeId(2))],
            },
            CompiledValueShape::Enum {
                // 5: Enum variant 2: Structured [Struct (3), Enum (4)]
                variants: vec![
                    CompiledEnumValueShape::Simple, // 0
                    CompiledEnumValueShape::Simple, // 1
                    CompiledEnumValueShape::Structured {
                        // 2
                        fields: vec![CompiledValueShapeId(3), CompiledValueShapeId(4)],
                    },
                ],
            },
        ];

        let valid_val = OwnedValue::Enum {
            variant: 2,
            payload: OwnedEnumPayload::Structured {
                fields: vec![
                    OwnedValue::Struct(
                        vec![
                            OwnedValue::String("hello".to_string().into_boxed_str()),
                            OwnedValue::Dynamic(OwnedDynamicValue::Float32(3.5)),
                        ]
                        .into_boxed_slice(),
                    ),
                    OwnedValue::Enum {
                        variant: 0,
                        payload: OwnedEnumPayload::Associated(Box::new(OwnedValue::Int64(999))),
                    },
                ]
                .into_boxed_slice(),
            },
        };

        let invalid_deep_leaf = OwnedValue::Enum {
            variant: 2,
            payload: OwnedEnumPayload::Structured {
                fields: vec![
                    OwnedValue::Struct(
                        vec![
                            OwnedValue::String("hello".to_string().into_boxed_str()),
                            OwnedValue::Dynamic(OwnedDynamicValue::Float32(3.5)),
                        ]
                        .into_boxed_slice(),
                    ),
                    OwnedValue::Enum {
                        variant: 0,
                        payload: OwnedEnumPayload::Associated(Box::new(OwnedValue::Int32(999))), // Mismatch here: Int32 instead of Int64
                    },
                ]
                .into_boxed_slice(),
            },
        };

        assert!(matches_owned_value_shape(
            &valid_val,
            CompiledValueShapeId(5),
            &shapes
        ));
        assert!(!matches_owned_value_shape(
            &invalid_deep_leaf,
            CompiledValueShapeId(5),
            &shapes
        ));
    }

    #[test]
    #[should_panic(expected = "CompiledValueShapeId must reference compiled value shape")]
    fn dangling_root_shape_id_is_invariant_violation() {
        let shapes = [CompiledValueShape::Boolean];
        let val = OwnedValue::Boolean(true);
        matches_owned_value_shape(&val, CompiledValueShapeId(99), &shapes);
    }

    #[test]
    #[should_panic(expected = "CompiledValueShapeId must reference compiled value shape")]
    fn dangling_nested_shape_id_is_invariant_violation() {
        let shapes = [CompiledValueShape::Struct {
            fields: vec![CompiledValueShapeId(99)],
        }];
        let val = OwnedValue::Struct(vec![OwnedValue::Boolean(true)].into_boxed_slice());
        matches_owned_value_shape(&val, CompiledValueShapeId(0), &shapes);
    }
}
