use alloc::vec::Vec;

use crate::data::compiled::identities::CompiledValueShapeId;

pub(crate) enum CompiledValueShape {
    Boolean,

    Int8,
    Int16,
    Int32,
    Int64,
    Int128,

    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,

    Float32,
    Float64,

    String,
    Dynamic,

    Struct {
        fields: Vec<CompiledValueShapeId>,
    },

    Enum {
        variants: Vec<CompiledEnumValueShape>,
    },
}

pub(crate) enum CompiledEnumValueShape {
    Simple,
    Associated(CompiledValueShapeId),
    Structured { fields: Vec<CompiledValueShapeId> },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiled_value_shape_17_variants() {
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
            CompiledValueShape::Struct {
                fields: alloc::vec![CompiledValueShapeId(0), CompiledValueShapeId(1)],
            },
            CompiledValueShape::Enum {
                variants: alloc::vec![
                    CompiledEnumValueShape::Simple,
                    CompiledEnumValueShape::Associated(CompiledValueShapeId(0)),
                    CompiledEnumValueShape::Structured {
                        fields: alloc::vec![CompiledValueShapeId(1), CompiledValueShapeId(2)],
                    },
                ],
            },
        ];

        assert_eq!(shapes.len(), 17);

        match &shapes[0] {
            CompiledValueShape::Boolean => {}
            _ => panic!("expected Boolean"),
        }
        match &shapes[14] {
            CompiledValueShape::Dynamic => {}
            _ => panic!("expected Dynamic"),
        }
        match &shapes[15] {
            CompiledValueShape::Struct { fields } => {
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].0, 0);
                assert_eq!(fields[1].0, 1);
            }
            _ => panic!("expected Struct"),
        }
        match &shapes[16] {
            CompiledValueShape::Enum { variants } => {
                assert_eq!(variants.len(), 3);
                match &variants[0] {
                    CompiledEnumValueShape::Simple => {}
                    _ => panic!("expected Simple variant"),
                }
                match &variants[1] {
                    CompiledEnumValueShape::Associated(id) => assert_eq!(id.0, 0),
                    _ => panic!("expected Associated variant"),
                }
                match &variants[2] {
                    CompiledEnumValueShape::Structured { fields } => {
                        assert_eq!(fields.len(), 2);
                        assert_eq!(fields[0].0, 1);
                        assert_eq!(fields[1].0, 2);
                    }
                    _ => panic!("expected Structured variant"),
                }
            }
            _ => panic!("expected Enum"),
        }
    }
}
