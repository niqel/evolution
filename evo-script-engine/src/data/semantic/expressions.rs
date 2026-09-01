use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use crate::data::ast::expressions::{BinaryOperator, UnaryOperator};
use crate::data::lexical::SourceSpan;
use crate::data::semantic::ids::{
    BindingId, FieldId, FunctionId, SignatureBindingId, SignatureId, TypeId, VariantId,
};

pub(crate) struct SemanticFunctionBody {
    pub(crate) statements: Vec<SemanticStatement>,
    pub(crate) result: SemanticExpression,
}

pub(crate) enum SemanticStatement {
    Bind {
        binding: BindingId,
        value: SemanticExpression,
    },
    Operation(SemanticExpression),
}

pub(crate) struct SemanticExpression {
    pub(crate) type_id: TypeId,
    pub(crate) kind: SemanticExpressionKind,
    pub(crate) span: SourceSpan,
}

pub(crate) enum SemanticExpressionKind {
    Literal(SemanticLiteral),
    Binding(BindingId),
    Unary {
        operator: UnaryOperator,
        operand: Box<SemanticExpression>,
    },
    Binary {
        left: Box<SemanticExpression>,
        operator: BinaryOperator,
        right: Box<SemanticExpression>,
    },
    Conversion {
        operand: Box<SemanticExpression>,
    },
    FieldAccess {
        receiver: Box<SemanticExpression>,
        field: FieldId,
    },
    Call(SemanticCall),
    StructConstruction {
        fields: Vec<SemanticFieldValue>,
    },
    EnumConstruction {
        variant: VariantId,
        payload: SemanticEnumPayload,
    },
    When(SemanticWhen),
}

pub(crate) enum SemanticLiteral {
    Integer(String),
    Floating(f64),
    Boolean(bool),
    String(String),
}

pub(crate) enum SemanticCallTarget {
    Internal(FunctionId),
    DirectSignature(SignatureId),
    SignatureDependency(SignatureBindingId),
}

pub(crate) enum SemanticArgument {
    Value(SemanticExpression),
    SignatureDependency(SignatureBindingId),
}

pub(crate) struct SemanticCall {
    pub(crate) target: SemanticCallTarget,
    pub(crate) arguments: Vec<SemanticArgument>,
}

pub(crate) struct SemanticFieldValue {
    pub(crate) field: FieldId,
    pub(crate) value: SemanticExpression,
}

pub(crate) enum SemanticEnumPayload {
    Simple,
    Associated { value: Box<SemanticExpression> },
    Structured { fields: Vec<SemanticFieldValue> },
}

pub(crate) struct SemanticWhen {
    pub(crate) subject: Box<SemanticExpression>,
    pub(crate) branches: Vec<SemanticWhenBranch>,
}

pub(crate) struct SemanticWhenBranch {
    pub(crate) variant: VariantId,
    pub(crate) extraction: SemanticVariantExtraction,
    pub(crate) result: SemanticExpression,
}

pub(crate) enum SemanticVariantExtraction {
    Simple,
    Associated { binding: BindingId },
    Structured { fields: Vec<SemanticFieldBinding> },
}

pub(crate) struct SemanticFieldBinding {
    pub(crate) field: FieldId,
    pub(crate) binding: BindingId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn semantic_literal_variants_and_large_integer() {
        let l_int =
            SemanticLiteral::Integer("34028236692093846346337460743176821145600000".to_string());
        match &l_int {
            SemanticLiteral::Integer(s) => {
                assert_eq!(s, "34028236692093846346337460743176821145600000");
            }
            _ => panic!("expected Integer"),
        }

        let l_float = SemanticLiteral::Floating(42.5);
        match l_float {
            SemanticLiteral::Floating(f) => {
                assert!((f - 42.5).abs() < 1e-10);
            }
            _ => panic!("expected Floating"),
        }

        let l_bool = SemanticLiteral::Boolean(true);
        match l_bool {
            SemanticLiteral::Boolean(b) => assert!(b),
            _ => panic!("expected Boolean"),
        }

        let l_str = SemanticLiteral::String("hello world".to_string());
        match &l_str {
            SemanticLiteral::String(s) => assert_eq!(s, "hello world"),
            _ => panic!("expected String"),
        }
    }

    #[test]
    fn semantic_expression_kind_10_variants() {
        let dummy_span = SourceSpan { start: 0, end: 1 };
        let dummy_expr = SemanticExpression {
            type_id: TypeId(0),
            kind: SemanticExpressionKind::Literal(SemanticLiteral::Boolean(true)),
            span: dummy_span,
        };

        let e_lit = SemanticExpressionKind::Literal(SemanticLiteral::Integer("42".to_string()));
        match &e_lit {
            SemanticExpressionKind::Literal(SemanticLiteral::Integer(s)) => assert_eq!(s, "42"),
            _ => panic!("expected Literal"),
        }

        let e_bind = SemanticExpressionKind::Binding(BindingId(0));
        match e_bind {
            SemanticExpressionKind::Binding(bid) => assert_eq!(bid.0, 0),
            _ => panic!("expected Binding"),
        }

        let e_un = SemanticExpressionKind::Unary {
            operator: UnaryOperator::Not,
            operand: Box::new(dummy_expr),
        };
        match e_un {
            SemanticExpressionKind::Unary { operator, operand } => {
                match operator {
                    UnaryOperator::Not => {}
                    _ => panic!("expected Not"),
                }
                assert_eq!(operand.type_id.0, 0);
            }
            _ => panic!("expected Unary"),
        }

        let e_bin = SemanticExpressionKind::Binary {
            left: Box::new(SemanticExpression {
                type_id: TypeId(0),
                kind: SemanticExpressionKind::Literal(SemanticLiteral::Integer("1".to_string())),
                span: dummy_span,
            }),
            operator: BinaryOperator::Add,
            right: Box::new(SemanticExpression {
                type_id: TypeId(0),
                kind: SemanticExpressionKind::Literal(SemanticLiteral::Integer("2".to_string())),
                span: dummy_span,
            }),
        };
        match e_bin {
            SemanticExpressionKind::Binary {
                left,
                operator,
                right,
            } => {
                assert_eq!(left.type_id.0, 0);
                match operator {
                    BinaryOperator::Add => {}
                    _ => panic!("expected Add"),
                }
                assert_eq!(right.type_id.0, 0);
            }
            _ => panic!("expected Binary"),
        }

        let e_conv = SemanticExpressionKind::Conversion {
            operand: Box::new(SemanticExpression {
                type_id: TypeId(0),
                kind: SemanticExpressionKind::Literal(SemanticLiteral::Integer("10".to_string())),
                span: dummy_span,
            }),
        };
        match e_conv {
            SemanticExpressionKind::Conversion { operand } => {
                assert_eq!(operand.type_id.0, 0);
            }
            _ => panic!("expected Conversion"),
        }

        let e_fa = SemanticExpressionKind::FieldAccess {
            receiver: Box::new(SemanticExpression {
                type_id: TypeId(0),
                kind: SemanticExpressionKind::Binding(BindingId(0)),
                span: dummy_span,
            }),
            field: FieldId(2),
        };
        match e_fa {
            SemanticExpressionKind::FieldAccess { receiver, field } => {
                assert_eq!(receiver.type_id.0, 0);
                assert_eq!(field.0, 2);
            }
            _ => panic!("expected FieldAccess"),
        }

        let e_call = SemanticExpressionKind::Call(SemanticCall {
            target: SemanticCallTarget::Internal(FunctionId(0)),
            arguments: alloc::vec![],
        });
        match e_call {
            SemanticExpressionKind::Call(c) => match c.target {
                SemanticCallTarget::Internal(fid) => assert_eq!(fid.0, 0),
                _ => panic!("expected Internal"),
            },
            _ => panic!("expected Call"),
        }

        let e_sc = SemanticExpressionKind::StructConstruction {
            fields: alloc::vec![SemanticFieldValue {
                field: FieldId(0),
                value: SemanticExpression {
                    type_id: TypeId(0),
                    kind: SemanticExpressionKind::Literal(SemanticLiteral::Integer(
                        "100".to_string(),
                    )),
                    span: dummy_span,
                },
            }],
        };
        match e_sc {
            SemanticExpressionKind::StructConstruction { fields } => {
                assert_eq!(fields.len(), 1);
                assert_eq!(fields[0].field.0, 0);
            }
            _ => panic!("expected StructConstruction"),
        }

        let e_ec = SemanticExpressionKind::EnumConstruction {
            variant: VariantId(0),
            payload: SemanticEnumPayload::Simple,
        };
        match e_ec {
            SemanticExpressionKind::EnumConstruction { variant, payload } => {
                assert_eq!(variant.0, 0);
                match payload {
                    SemanticEnumPayload::Simple => {}
                    _ => panic!("expected Simple"),
                }
            }
            _ => panic!("expected EnumConstruction"),
        }

        let e_when = SemanticExpressionKind::When(SemanticWhen {
            subject: Box::new(SemanticExpression {
                type_id: TypeId(0),
                kind: SemanticExpressionKind::Binding(BindingId(0)),
                span: dummy_span,
            }),
            branches: alloc::vec![SemanticWhenBranch {
                variant: VariantId(0),
                extraction: SemanticVariantExtraction::Simple,
                result: SemanticExpression {
                    type_id: TypeId(1),
                    kind: SemanticExpressionKind::Literal(SemanticLiteral::Integer(
                        "0".to_string()
                    )),
                    span: dummy_span,
                },
            }],
        });
        match e_when {
            SemanticExpressionKind::When(w) => {
                assert_eq!(w.branches.len(), 1);
                assert_eq!(w.branches[0].variant.0, 0);
            }
            _ => panic!("expected When"),
        }
    }

    #[test]
    fn semantic_call_target_variants() {
        let t_internal = SemanticCallTarget::Internal(FunctionId(1));
        match t_internal {
            SemanticCallTarget::Internal(fid) => assert_eq!(fid.0, 1),
            _ => panic!("expected Internal"),
        }

        let t_sig = SemanticCallTarget::DirectSignature(SignatureId(2));
        match t_sig {
            SemanticCallTarget::DirectSignature(sid) => assert_eq!(sid.0, 2),
            _ => panic!("expected DirectSignature"),
        }

        let t_dep = SemanticCallTarget::SignatureDependency(SignatureBindingId(3));
        match t_dep {
            SemanticCallTarget::SignatureDependency(sbid) => assert_eq!(sbid.0, 3),
            _ => panic!("expected SignatureDependency"),
        }
    }

    #[test]
    fn semantic_call_arguments_order() {
        let span = SourceSpan { start: 0, end: 1 };
        let call = SemanticCall {
            target: SemanticCallTarget::Internal(FunctionId(0)),
            arguments: alloc::vec![
                SemanticArgument::Value(SemanticExpression {
                    type_id: TypeId(0),
                    kind: SemanticExpressionKind::Literal(SemanticLiteral::Integer(
                        "1".to_string()
                    )),
                    span,
                }),
                SemanticArgument::SignatureDependency(SignatureBindingId(5)),
            ],
        };

        assert_eq!(call.arguments.len(), 2);
        match &call.arguments[0] {
            SemanticArgument::Value(expr) => match &expr.kind {
                SemanticExpressionKind::Literal(SemanticLiteral::Integer(s)) => {
                    assert_eq!(s, "1");
                }
                _ => panic!("expected Integer literal"),
            },
            _ => panic!("expected Value at [0]"),
        }
        match &call.arguments[1] {
            SemanticArgument::SignatureDependency(sbid) => assert_eq!(sbid.0, 5),
            _ => panic!("expected SignatureDependency at [1]"),
        }
    }

    #[test]
    fn semantic_enum_payload_variants() {
        let span = SourceSpan { start: 0, end: 1 };
        let p_simple = SemanticEnumPayload::Simple;
        match p_simple {
            SemanticEnumPayload::Simple => {}
            _ => panic!("expected Simple"),
        }

        let p_assoc = SemanticEnumPayload::Associated {
            value: Box::new(SemanticExpression {
                type_id: TypeId(0),
                kind: SemanticExpressionKind::Literal(SemanticLiteral::Integer("42".to_string())),
                span,
            }),
        };
        match p_assoc {
            SemanticEnumPayload::Associated { value } => {
                assert_eq!(value.type_id.0, 0);
            }
            _ => panic!("expected Associated"),
        }

        let p_struct = SemanticEnumPayload::Structured {
            fields: alloc::vec![SemanticFieldValue {
                field: FieldId(0),
                value: SemanticExpression {
                    type_id: TypeId(0),
                    kind: SemanticExpressionKind::Literal(SemanticLiteral::Boolean(false)),
                    span,
                },
            }],
        };
        match p_struct {
            SemanticEnumPayload::Structured { fields } => {
                assert_eq!(fields.len(), 1);
                assert_eq!(fields[0].field.0, 0);
            }
            _ => panic!("expected Structured"),
        }
    }

    #[test]
    fn semantic_when_branches_order() {
        let span = SourceSpan { start: 0, end: 1 };
        let when = SemanticWhen {
            subject: Box::new(SemanticExpression {
                type_id: TypeId(0),
                kind: SemanticExpressionKind::Binding(BindingId(0)),
                span,
            }),
            branches: alloc::vec![
                SemanticWhenBranch {
                    variant: VariantId(0),
                    extraction: SemanticVariantExtraction::Simple,
                    result: SemanticExpression {
                        type_id: TypeId(1),
                        kind: SemanticExpressionKind::Literal(SemanticLiteral::Integer(
                            "10".to_string(),
                        )),
                        span,
                    },
                },
                SemanticWhenBranch {
                    variant: VariantId(1),
                    extraction: SemanticVariantExtraction::Associated {
                        binding: BindingId(1),
                    },
                    result: SemanticExpression {
                        type_id: TypeId(1),
                        kind: SemanticExpressionKind::Literal(SemanticLiteral::Integer(
                            "20".to_string(),
                        )),
                        span,
                    },
                },
            ],
        };

        assert_eq!(when.branches.len(), 2);
        assert_eq!(when.branches[0].variant.0, 0);
        match &when.branches[0].extraction {
            SemanticVariantExtraction::Simple => {}
            _ => panic!("expected Simple at branch 0"),
        }
        assert_eq!(when.branches[1].variant.0, 1);
        match &when.branches[1].extraction {
            SemanticVariantExtraction::Associated { binding } => {
                assert_eq!(binding.0, 1);
            }
            _ => panic!("expected Associated at branch 1"),
        }
    }

    #[test]
    fn semantic_variant_extraction_variants_and_structured_order() {
        let ex_simple = SemanticVariantExtraction::Simple;
        match ex_simple {
            SemanticVariantExtraction::Simple => {}
            _ => panic!("expected Simple"),
        }

        let ex_assoc = SemanticVariantExtraction::Associated {
            binding: BindingId(3),
        };
        match ex_assoc {
            SemanticVariantExtraction::Associated { binding } => {
                assert_eq!(binding.0, 3);
            }
            _ => panic!("expected Associated"),
        }

        let ex_struct = SemanticVariantExtraction::Structured {
            fields: alloc::vec![
                SemanticFieldBinding {
                    field: FieldId(0),
                    binding: BindingId(10),
                },
                SemanticFieldBinding {
                    field: FieldId(1),
                    binding: BindingId(11),
                },
            ],
        };
        match ex_struct {
            SemanticVariantExtraction::Structured { fields } => {
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].field.0, 0);
                assert_eq!(fields[0].binding.0, 10);
                assert_eq!(fields[1].field.0, 1);
                assert_eq!(fields[1].binding.0, 11);
            }
            _ => panic!("expected Structured"),
        }
    }

    #[test]
    fn semantic_function_body_and_statements_sequence() {
        let span = SourceSpan { start: 0, end: 1 };
        let body = SemanticFunctionBody {
            statements: alloc::vec![
                SemanticStatement::Bind {
                    binding: BindingId(0),
                    value: SemanticExpression {
                        type_id: TypeId(0),
                        kind: SemanticExpressionKind::Literal(SemanticLiteral::Integer(
                            "100".to_string(),
                        )),
                        span,
                    },
                },
                SemanticStatement::Operation(SemanticExpression {
                    type_id: TypeId(1),
                    kind: SemanticExpressionKind::Call(SemanticCall {
                        target: SemanticCallTarget::Internal(FunctionId(1)),
                        arguments: alloc::vec![],
                    }),
                    span,
                }),
            ],
            result: SemanticExpression {
                type_id: TypeId(0),
                kind: SemanticExpressionKind::Binding(BindingId(0)),
                span,
            },
        };

        assert_eq!(body.statements.len(), 2);
        match &body.statements[0] {
            SemanticStatement::Bind { binding, value } => {
                assert_eq!(binding.0, 0);
                assert_eq!(value.type_id.0, 0);
            }
            _ => panic!("expected Bind"),
        }
        match &body.statements[1] {
            SemanticStatement::Operation(expr) => {
                assert_eq!(expr.type_id.0, 1);
            }
            _ => panic!("expected Operation"),
        }
        assert_eq!(body.result.type_id.0, 0);
    }
}
