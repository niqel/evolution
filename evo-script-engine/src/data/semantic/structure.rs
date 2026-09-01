use alloc::vec::Vec;

use crate::data::semantic::SignatureSymbol;
use crate::data::semantic::expressions::SemanticFunctionBody;
use crate::data::semantic::ids::{BindingId, FunctionId, SignatureBindingId, SignatureId, TypeId};

pub(crate) enum NativeType {
    Int,
    Float,
    Bool,
    String,
    Dynamic,

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
}

pub(crate) enum SemanticType {
    Native(NativeType),
    Struct { fields: Vec<SemanticField> },
    Enum { variants: Vec<SemanticVariant> },
}

pub(crate) struct SemanticField {
    pub(crate) type_id: TypeId,
}

pub(crate) enum SemanticVariant {
    Simple,
    Associated { type_id: TypeId },
    Structured { fields: Vec<SemanticField> },
}

pub(crate) struct SemanticBinding {
    pub(crate) type_id: TypeId,
}

pub(crate) struct SemanticSignatureBinding {
    pub(crate) signature: SignatureId,
}

pub(crate) enum SemanticParameter {
    Value(BindingId),
    SignatureDependency(SignatureBindingId),
}

pub(crate) enum SemanticSignatureParameter {
    Value(TypeId),
    SignatureDependency(SignatureId),
}

pub(crate) struct SemanticSignature {
    pub(crate) symbol: SignatureSymbol,
    pub(crate) parameters: Vec<SemanticSignatureParameter>,
    pub(crate) result_type: TypeId,
}

pub(crate) struct SemanticFunction {
    pub(crate) parameters: Vec<SemanticParameter>,
    pub(crate) bindings: Vec<SemanticBinding>,
    pub(crate) signature_bindings: Vec<SemanticSignatureBinding>,
    pub(crate) result_type: TypeId,
    pub(crate) satisfaction: Option<SignatureId>,
    pub(crate) body: SemanticFunctionBody,
}

pub(crate) struct SemanticProgram {
    pub(crate) types: Vec<SemanticType>,
    pub(crate) signatures: Vec<SemanticSignature>,
    pub(crate) functions: Vec<SemanticFunction>,
    pub(crate) entry_function: FunctionId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::lexical::SourceSpan;
    use crate::data::semantic::expressions::{
        SemanticExpression, SemanticExpressionKind, SemanticLiteral,
    };
    use alloc::string::ToString;

    #[test]
    fn native_type_17_variants() {
        let types = [
            NativeType::Int,
            NativeType::Float,
            NativeType::Bool,
            NativeType::String,
            NativeType::Dynamic,
            NativeType::Int8,
            NativeType::Int16,
            NativeType::Int32,
            NativeType::Int64,
            NativeType::Int128,
            NativeType::Uint8,
            NativeType::Uint16,
            NativeType::Uint32,
            NativeType::Uint64,
            NativeType::Uint128,
            NativeType::Float32,
            NativeType::Float64,
        ];

        assert_eq!(types.len(), 17);
        match &types[0] {
            NativeType::Int => {}
            _ => panic!("expected Int"),
        }
        match &types[16] {
            NativeType::Float64 => {}
            _ => panic!("expected Float64"),
        }
    }

    #[test]
    fn semantic_type_and_variants_order() {
        let t_native = SemanticType::Native(NativeType::Int32);
        match t_native {
            SemanticType::Native(nt) => match nt {
                NativeType::Int32 => {}
                _ => panic!("expected Int32"),
            },
            _ => panic!("expected Native"),
        }

        let t_struct = SemanticType::Struct {
            fields: alloc::vec![
                SemanticField { type_id: TypeId(0) },
                SemanticField { type_id: TypeId(1) },
            ],
        };
        match t_struct {
            SemanticType::Struct { fields } => {
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].type_id.0, 0);
                assert_eq!(fields[1].type_id.0, 1);
            }
            _ => panic!("expected Struct"),
        }

        let t_enum = SemanticType::Enum {
            variants: alloc::vec![
                SemanticVariant::Simple,
                SemanticVariant::Associated { type_id: TypeId(0) },
                SemanticVariant::Structured {
                    fields: alloc::vec![SemanticField { type_id: TypeId(1) }],
                },
            ],
        };
        match t_enum {
            SemanticType::Enum { variants } => {
                assert_eq!(variants.len(), 3);
                match &variants[0] {
                    SemanticVariant::Simple => {}
                    _ => panic!("expected Simple at [0]"),
                }
                match &variants[1] {
                    SemanticVariant::Associated { type_id } => {
                        assert_eq!(type_id.0, 0);
                    }
                    _ => panic!("expected Associated at [1]"),
                }
                match &variants[2] {
                    SemanticVariant::Structured { fields } => {
                        assert_eq!(fields.len(), 1);
                        assert_eq!(fields[0].type_id.0, 1);
                    }
                    _ => panic!("expected Structured at [2]"),
                }
            }
            _ => panic!("expected Enum"),
        }
    }

    #[test]
    fn semantic_parameter_mixed_order() {
        let params = alloc::vec![
            SemanticParameter::Value(BindingId(0)),
            SemanticParameter::SignatureDependency(SignatureBindingId(0)),
            SemanticParameter::Value(BindingId(1)),
        ];

        assert_eq!(params.len(), 3);
        match &params[0] {
            SemanticParameter::Value(bid) => assert_eq!(bid.0, 0),
            _ => panic!("expected Value at [0]"),
        }
        match &params[1] {
            SemanticParameter::SignatureDependency(sbid) => assert_eq!(sbid.0, 0),
            _ => panic!("expected SignatureDependency at [1]"),
        }
        match &params[2] {
            SemanticParameter::Value(bid) => assert_eq!(bid.0, 1),
            _ => panic!("expected Value at [2]"),
        }
    }

    #[test]
    fn semantic_signature_parameter_mixed_order() {
        let params = alloc::vec![
            SemanticSignatureParameter::Value(TypeId(0)),
            SemanticSignatureParameter::SignatureDependency(SignatureId(0)),
            SemanticSignatureParameter::Value(TypeId(1)),
        ];

        assert_eq!(params.len(), 3);
        match &params[0] {
            SemanticSignatureParameter::Value(tid) => assert_eq!(tid.0, 0),
            _ => panic!("expected Value at [0]"),
        }
        match &params[1] {
            SemanticSignatureParameter::SignatureDependency(sid) => assert_eq!(sid.0, 0),
            _ => panic!("expected SignatureDependency at [1]"),
        }
        match &params[2] {
            SemanticSignatureParameter::Value(tid) => assert_eq!(tid.0, 1),
            _ => panic!("expected Value at [2]"),
        }
    }

    #[test]
    fn semantic_signature_fields() {
        let sig = SemanticSignature {
            symbol: SignatureSymbol {
                module: "Math".to_string(),
                name: "Add".to_string(),
            },
            parameters: alloc::vec![
                SemanticSignatureParameter::Value(TypeId(0)),
                SemanticSignatureParameter::Value(TypeId(0)),
            ],
            result_type: TypeId(0),
        };

        assert_eq!(sig.symbol.module, "Math");
        assert_eq!(sig.symbol.name, "Add");
        assert_eq!(sig.parameters.len(), 2);
        assert_eq!(sig.result_type.0, 0);
    }

    #[test]
    fn semantic_function_owner_index_relationships() {
        let span = SourceSpan { start: 0, end: 1 };
        let func = SemanticFunction {
            parameters: alloc::vec![
                SemanticParameter::Value(BindingId(0)),
                SemanticParameter::SignatureDependency(SignatureBindingId(0)),
            ],
            bindings: alloc::vec![SemanticBinding { type_id: TypeId(0) }],
            signature_bindings: alloc::vec![SemanticSignatureBinding {
                signature: SignatureId(0),
            }],
            result_type: TypeId(0),
            satisfaction: Some(SignatureId(0)),
            body: SemanticFunctionBody {
                statements: alloc::vec![],
                result: SemanticExpression {
                    type_id: TypeId(0),
                    kind: SemanticExpressionKind::Literal(SemanticLiteral::Integer(
                        "0".to_string(),
                    )),
                    span,
                },
            },
        };

        assert_eq!(func.parameters.len(), 2);
        assert_eq!(func.bindings.len(), 1);
        assert_eq!(func.bindings[0].type_id.0, 0);
        assert_eq!(func.signature_bindings.len(), 1);
        assert_eq!(func.signature_bindings[0].signature.0, 0);
        assert_eq!(func.result_type.0, 0);
        match func.satisfaction {
            Some(sid) => assert_eq!(sid.0, 0),
            None => panic!("expected Some satisfaction"),
        }
    }

    #[test]
    fn semantic_program_minimal_valid() {
        let span = SourceSpan { start: 0, end: 1 };
        let program = SemanticProgram {
            types: alloc::vec![
                SemanticType::Native(NativeType::Int32),
                SemanticType::Native(NativeType::Bool),
            ],
            signatures: alloc::vec![SemanticSignature {
                symbol: SignatureSymbol {
                    module: "Math".to_string(),
                    name: "Add".to_string(),
                },
                parameters: alloc::vec![
                    SemanticSignatureParameter::Value(TypeId(0)),
                    SemanticSignatureParameter::Value(TypeId(0)),
                ],
                result_type: TypeId(0),
            }],
            functions: alloc::vec![SemanticFunction {
                parameters: alloc::vec![],
                bindings: alloc::vec![],
                signature_bindings: alloc::vec![],
                result_type: TypeId(0),
                satisfaction: None,
                body: SemanticFunctionBody {
                    statements: alloc::vec![],
                    result: SemanticExpression {
                        type_id: TypeId(0),
                        kind: SemanticExpressionKind::Literal(SemanticLiteral::Integer(
                            "42".to_string()
                        )),
                        span,
                    },
                },
            }],
            entry_function: FunctionId(0),
        };

        assert_eq!(program.types.len(), 2);
        assert_eq!(program.signatures.len(), 1);
        assert_eq!(program.functions.len(), 1);
        assert_eq!(program.entry_function.0, 0);
    }
}
