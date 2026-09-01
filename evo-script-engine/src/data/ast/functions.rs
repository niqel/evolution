use alloc::vec::Vec;

use crate::data::ast::expressions::{Expression, FunctionCall, Pipeline};
use crate::data::ast::foundational::{Identifier, QualifiedName, TypedBinding, Visibility};

pub(crate) struct FunctionDefinition<'source> {
    pub(crate) visibility: Visibility,
    pub(crate) name: Identifier<'source>,
    pub(crate) parameters: Vec<Parameter<'source>>,
    pub(crate) result_type: Identifier<'source>,
    pub(crate) satisfaction: Option<QualifiedName<'source>>,
    pub(crate) body: FunctionBody<'source>,
}

pub(crate) enum Parameter<'source> {
    Value(TypedBinding<'source>),
    SignatureDependency {
        signature: QualifiedName<'source>,
        name: Identifier<'source>,
    },
}

pub(crate) struct FunctionBody<'source> {
    pub(crate) statements: Vec<BodyStatement<'source>>,
    pub(crate) result: Expression<'source>,
}

pub(crate) enum BodyStatement<'source> {
    Let(LetBinding<'source>),
    Operation(OperationStatement<'source>),
}

pub(crate) struct LetBinding<'source> {
    pub(crate) binding: TypedBinding<'source>,
    pub(crate) value: Expression<'source>,
}

pub(crate) enum OperationStatement<'source> {
    FunctionCall(FunctionCall<'source>),
    Pipeline(Pipeline<'source>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::ast::expressions::{ExpressionKind, LiteralKind, PipelineStage};
    use crate::data::lexical::SourceSpan;
    use alloc::boxed::Box;
    use alloc::string::String as AllocString;

    #[test]
    fn parameter_variants() {
        let source = AllocString::from("int32 count Math::Adder adder");
        let p_val = Parameter::Value(TypedBinding {
            type_name: Identifier {
                lexeme: &source[0..5],
                span: SourceSpan { start: 0, end: 5 },
            },
            name: Identifier {
                lexeme: &source[6..11],
                span: SourceSpan { start: 6, end: 11 },
            },
        });
        match p_val {
            Parameter::Value(b) => {
                assert_eq!(b.type_name.lexeme, "int32");
                assert_eq!(b.name.lexeme, "count");
            }
            _ => panic!("expected Value"),
        }

        let p_sig = Parameter::SignatureDependency {
            signature: QualifiedName {
                qualifier: Identifier {
                    lexeme: &source[12..16],
                    span: SourceSpan { start: 12, end: 16 },
                },
                name: Identifier {
                    lexeme: &source[18..23],
                    span: SourceSpan { start: 18, end: 23 },
                },
            },
            name: Identifier {
                lexeme: &source[24..29],
                span: SourceSpan { start: 24, end: 29 },
            },
        };
        match p_sig {
            Parameter::SignatureDependency { signature, name } => {
                assert_eq!(signature.qualifier.lexeme, "Math");
                assert_eq!(signature.name.lexeme, "Adder");
                assert_eq!(name.lexeme, "adder");
            }
            _ => panic!("expected SignatureDependency"),
        }
    }

    #[test]
    fn function_definition_mixed_parameter_order() {
        let source = AllocString::from("int32 a Math::Adder b int32 c 0");
        let func = FunctionDefinition {
            visibility: Visibility::Public,
            name: Identifier {
                lexeme: &source[0..5],
                span: SourceSpan { start: 0, end: 5 },
            },
            parameters: alloc::vec![
                Parameter::Value(TypedBinding {
                    type_name: Identifier {
                        lexeme: &source[0..5],
                        span: SourceSpan { start: 0, end: 5 },
                    },
                    name: Identifier {
                        lexeme: &source[6..7],
                        span: SourceSpan { start: 6, end: 7 },
                    },
                }),
                Parameter::SignatureDependency {
                    signature: QualifiedName {
                        qualifier: Identifier {
                            lexeme: &source[8..12],
                            span: SourceSpan { start: 8, end: 12 },
                        },
                        name: Identifier {
                            lexeme: &source[14..19],
                            span: SourceSpan { start: 14, end: 19 },
                        },
                    },
                    name: Identifier {
                        lexeme: &source[20..21],
                        span: SourceSpan { start: 20, end: 21 },
                    },
                },
                Parameter::Value(TypedBinding {
                    type_name: Identifier {
                        lexeme: &source[22..27],
                        span: SourceSpan { start: 22, end: 27 },
                    },
                    name: Identifier {
                        lexeme: &source[28..29],
                        span: SourceSpan { start: 28, end: 29 },
                    },
                }),
            ],
            result_type: Identifier {
                lexeme: &source[0..5],
                span: SourceSpan { start: 0, end: 5 },
            },
            satisfaction: None,
            body: FunctionBody {
                statements: alloc::vec![],
                result: Expression {
                    kind: ExpressionKind::Literal {
                        kind: LiteralKind::Integer,
                        lexeme: &source[30..31],
                    },
                    span: SourceSpan { start: 30, end: 31 },
                },
            },
        };

        assert_eq!(func.parameters.len(), 3);
        match &func.parameters[0] {
            Parameter::Value(b) => {
                assert_eq!(b.type_name.lexeme, "int32");
                assert_eq!(b.name.lexeme, "a");
            }
            _ => panic!("expected Parameter::Value at [0]"),
        }
        match &func.parameters[1] {
            Parameter::SignatureDependency { signature, name } => {
                assert_eq!(signature.qualifier.lexeme, "Math");
                assert_eq!(signature.name.lexeme, "Adder");
                assert_eq!(name.lexeme, "b");
            }
            _ => panic!("expected Parameter::SignatureDependency at [1]"),
        }
        match &func.parameters[2] {
            Parameter::Value(b) => {
                assert_eq!(b.type_name.lexeme, "int32");
                assert_eq!(b.name.lexeme, "c");
            }
            _ => panic!("expected Parameter::Value at [2]"),
        }
    }

    #[test]
    fn function_definition_public_and_private_satisfaction() {
        let source = AllocString::from("compute int32 res 0 Math::Compute");
        let func_pub = FunctionDefinition {
            visibility: Visibility::Public,
            name: Identifier {
                lexeme: &source[0..7],
                span: SourceSpan { start: 0, end: 7 },
            },
            parameters: alloc::vec![],
            result_type: Identifier {
                lexeme: &source[8..13],
                span: SourceSpan { start: 8, end: 13 },
            },
            satisfaction: None,
            body: FunctionBody {
                statements: alloc::vec![],
                result: Expression {
                    kind: ExpressionKind::Literal {
                        kind: LiteralKind::Integer,
                        lexeme: &source[18..19],
                    },
                    span: SourceSpan { start: 18, end: 19 },
                },
            },
        };
        match func_pub.visibility {
            Visibility::Public => {}
            _ => panic!("expected Public"),
        }
        assert_eq!(func_pub.name.lexeme, "compute");
        assert!(func_pub.satisfaction.is_none());

        let func_priv = FunctionDefinition {
            visibility: Visibility::Private,
            name: Identifier {
                lexeme: &source[0..7],
                span: SourceSpan { start: 0, end: 7 },
            },
            parameters: alloc::vec![],
            result_type: Identifier {
                lexeme: &source[8..13],
                span: SourceSpan { start: 8, end: 13 },
            },
            satisfaction: Some(QualifiedName {
                qualifier: Identifier {
                    lexeme: &source[20..24],
                    span: SourceSpan { start: 20, end: 24 },
                },
                name: Identifier {
                    lexeme: &source[26..33],
                    span: SourceSpan { start: 26, end: 33 },
                },
            }),
            body: FunctionBody {
                statements: alloc::vec![],
                result: Expression {
                    kind: ExpressionKind::Literal {
                        kind: LiteralKind::Integer,
                        lexeme: &source[18..19],
                    },
                    span: SourceSpan { start: 18, end: 19 },
                },
            },
        };
        match func_priv.visibility {
            Visibility::Private => {}
            _ => panic!("expected Private"),
        }
        match func_priv.satisfaction {
            Some(sat) => {
                assert_eq!(sat.qualifier.lexeme, "Math");
                assert_eq!(sat.name.lexeme, "Compute");
            }
            None => panic!("expected Some satisfaction"),
        }
    }

    #[test]
    fn function_body_statements_and_result() {
        let source = AllocString::from("int32 x 42 log x f 0");
        let body = FunctionBody {
            statements: alloc::vec![
                BodyStatement::Let(LetBinding {
                    binding: TypedBinding {
                        type_name: Identifier {
                            lexeme: &source[0..5],
                            span: SourceSpan { start: 0, end: 5 },
                        },
                        name: Identifier {
                            lexeme: &source[6..7],
                            span: SourceSpan { start: 6, end: 7 },
                        },
                    },
                    value: Expression {
                        kind: ExpressionKind::Literal {
                            kind: LiteralKind::Integer,
                            lexeme: &source[8..10],
                        },
                        span: SourceSpan { start: 8, end: 10 },
                    },
                }),
                BodyStatement::Operation(OperationStatement::FunctionCall(FunctionCall {
                    callee: Identifier {
                        lexeme: &source[11..14],
                        span: SourceSpan { start: 11, end: 14 },
                    },
                    arguments: alloc::vec![Expression {
                        kind: ExpressionKind::Identifier(Identifier {
                            lexeme: &source[15..16],
                            span: SourceSpan { start: 15, end: 16 },
                        }),
                        span: SourceSpan { start: 15, end: 16 },
                    }],
                })),
                BodyStatement::Operation(OperationStatement::Pipeline(Pipeline {
                    source: Box::new(Expression {
                        kind: ExpressionKind::Identifier(Identifier {
                            lexeme: &source[15..16],
                            span: SourceSpan { start: 15, end: 16 },
                        }),
                        span: SourceSpan { start: 15, end: 16 },
                    }),
                    stages: alloc::vec![PipelineStage {
                        callee: Identifier {
                            lexeme: &source[17..18],
                            span: SourceSpan { start: 17, end: 18 },
                        },
                        additional_arguments: alloc::vec![],
                    }],
                })),
            ],
            result: Expression {
                kind: ExpressionKind::Literal {
                    kind: LiteralKind::Integer,
                    lexeme: &source[19..20],
                },
                span: SourceSpan { start: 19, end: 20 },
            },
        };

        assert_eq!(body.statements.len(), 3);
        match &body.statements[0] {
            BodyStatement::Let(lb) => assert_eq!(lb.binding.name.lexeme, "x"),
            _ => panic!("expected Let"),
        }
        match &body.statements[1] {
            BodyStatement::Operation(OperationStatement::FunctionCall(fc)) => {
                assert_eq!(fc.callee.lexeme, "log");
            }
            _ => panic!("expected FunctionCall"),
        }
        match &body.statements[2] {
            BodyStatement::Operation(OperationStatement::Pipeline(pip)) => {
                assert_eq!(pip.stages.len(), 1);
            }
            _ => panic!("expected Pipeline"),
        }
    }
}
