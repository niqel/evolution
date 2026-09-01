use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::data::ast::foundational::{Identifier, QualifiedName};
use crate::data::ast::when::WhenExpression;
use crate::data::lexical::SourceSpan;

pub(crate) struct Expression<'source> {
    pub(crate) kind: ExpressionKind<'source>,
    pub(crate) span: SourceSpan,
}

pub(crate) enum ExpressionKind<'source> {
    Literal {
        kind: LiteralKind,
        lexeme: &'source str,
    },
    Identifier(Identifier<'source>),
    Unary {
        operator: UnaryOperator,
        operand: Box<Expression<'source>>,
    },
    Binary {
        left: Box<Expression<'source>>,
        operator: BinaryOperator,
        right: Box<Expression<'source>>,
    },
    FieldAccess {
        receiver: Box<Expression<'source>>,
        field: Identifier<'source>,
    },
    FunctionCall(FunctionCall<'source>),
    StructConstruction {
        type_name: Identifier<'source>,
        fields: Vec<FieldInitializer<'source>>,
    },
    EnumConstruction(EnumConstruction<'source>),
    Pipeline(Pipeline<'source>),
    When(WhenExpression<'source>),
}

pub(crate) enum LiteralKind {
    Integer,
    Floating,
    String,
    Boolean,
}

pub(crate) enum UnaryOperator {
    Not,
    Negate,
}

pub(crate) enum BinaryOperator {
    Multiply,
    Divide,
    Remainder,
    Add,
    Subtract,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    And,
    Or,
}

pub(crate) struct FunctionCall<'source> {
    pub(crate) callee: Identifier<'source>,
    pub(crate) arguments: Vec<Expression<'source>>,
}

pub(crate) struct FieldInitializer<'source> {
    pub(crate) name: Identifier<'source>,
    pub(crate) value: Expression<'source>,
}

pub(crate) enum EnumConstruction<'source> {
    Simple {
        variant: QualifiedName<'source>,
    },
    Associated {
        variant: QualifiedName<'source>,
        value: Box<Expression<'source>>,
    },
    Structured {
        variant: QualifiedName<'source>,
        fields: Vec<FieldInitializer<'source>>,
    },
}

pub(crate) struct Pipeline<'source> {
    pub(crate) source: Box<Expression<'source>>,
    pub(crate) stages: Vec<PipelineStage<'source>>,
}

pub(crate) struct PipelineStage<'source> {
    pub(crate) callee: Identifier<'source>,
    pub(crate) additional_arguments: Vec<Expression<'source>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::ast::when::{WhenCorrespondence, WhenPattern};
    use alloc::string::String as AllocString;

    #[test]
    fn literal_kind_variants() {
        let l_int = LiteralKind::Integer;
        match l_int {
            LiteralKind::Integer => {}
            _ => panic!("expected Integer"),
        }
        let l_float = LiteralKind::Floating;
        match l_float {
            LiteralKind::Floating => {}
            _ => panic!("expected Floating"),
        }
        let l_str = LiteralKind::String;
        match l_str {
            LiteralKind::String => {}
            _ => panic!("expected String"),
        }
        let l_bool = LiteralKind::Boolean;
        match l_bool {
            LiteralKind::Boolean => {}
            _ => panic!("expected Boolean"),
        }
    }

    #[test]
    fn unary_operator_variants() {
        let u_not = UnaryOperator::Not;
        match u_not {
            UnaryOperator::Not => {}
            _ => panic!("expected Not"),
        }
        let u_neg = UnaryOperator::Negate;
        match u_neg {
            UnaryOperator::Negate => {}
            _ => panic!("expected Negate"),
        }
    }

    #[test]
    fn binary_operator_variants() {
        let ops = [
            BinaryOperator::Multiply,
            BinaryOperator::Divide,
            BinaryOperator::Remainder,
            BinaryOperator::Add,
            BinaryOperator::Subtract,
            BinaryOperator::Less,
            BinaryOperator::LessEqual,
            BinaryOperator::Greater,
            BinaryOperator::GreaterEqual,
            BinaryOperator::Equal,
            BinaryOperator::NotEqual,
            BinaryOperator::And,
            BinaryOperator::Or,
        ];
        assert_eq!(ops.len(), 13);
        match ops[0] {
            BinaryOperator::Multiply => {}
            _ => panic!("expected Multiply"),
        }
        match ops[12] {
            BinaryOperator::Or => {}
            _ => panic!("expected Or"),
        }
    }

    #[test]
    fn enum_construction_variants() {
        let source = AllocString::from("Status::Active 42 score: 100");
        let simple = EnumConstruction::Simple {
            variant: QualifiedName {
                qualifier: Identifier {
                    lexeme: &source[0..6],
                    span: SourceSpan { start: 0, end: 6 },
                },
                name: Identifier {
                    lexeme: &source[8..14],
                    span: SourceSpan { start: 8, end: 14 },
                },
            },
        };
        match simple {
            EnumConstruction::Simple { variant } => {
                assert_eq!(variant.qualifier.lexeme, "Status");
                assert_eq!(variant.name.lexeme, "Active");
            }
            _ => panic!("expected Simple"),
        }

        let associated = EnumConstruction::Associated {
            variant: QualifiedName {
                qualifier: Identifier {
                    lexeme: &source[0..6],
                    span: SourceSpan { start: 0, end: 6 },
                },
                name: Identifier {
                    lexeme: &source[8..14],
                    span: SourceSpan { start: 8, end: 14 },
                },
            },
            value: Box::new(Expression {
                kind: ExpressionKind::Literal {
                    kind: LiteralKind::Integer,
                    lexeme: &source[15..17],
                },
                span: SourceSpan { start: 15, end: 17 },
            }),
        };
        match associated {
            EnumConstruction::Associated { variant, value } => {
                assert_eq!(variant.name.lexeme, "Active");
                match value.kind {
                    ExpressionKind::Literal { kind, lexeme } => {
                        match kind {
                            LiteralKind::Integer => {}
                            _ => panic!("expected Integer"),
                        }
                        assert_eq!(lexeme, "42");
                    }
                    _ => panic!("expected Literal"),
                }
            }
            _ => panic!("expected Associated"),
        }

        let structured = EnumConstruction::Structured {
            variant: QualifiedName {
                qualifier: Identifier {
                    lexeme: &source[0..6],
                    span: SourceSpan { start: 0, end: 6 },
                },
                name: Identifier {
                    lexeme: &source[8..14],
                    span: SourceSpan { start: 8, end: 14 },
                },
            },
            fields: alloc::vec![FieldInitializer {
                name: Identifier {
                    lexeme: &source[18..23],
                    span: SourceSpan { start: 18, end: 23 },
                },
                value: Expression {
                    kind: ExpressionKind::Literal {
                        kind: LiteralKind::Integer,
                        lexeme: &source[25..28],
                    },
                    span: SourceSpan { start: 25, end: 28 },
                },
            }],
        };
        match structured {
            EnumConstruction::Structured { variant, fields } => {
                assert_eq!(variant.name.lexeme, "Active");
                assert_eq!(fields.len(), 1);
                assert_eq!(fields[0].name.lexeme, "score");
            }
            _ => panic!("expected Structured"),
        }
    }

    #[test]
    fn expression_kind_10_variants() {
        let source =
            AllocString::from("42 x -a a + b a.f call(a) User{a: 1} S::A x |> f Enum::Var 1");
        let e_lit = ExpressionKind::Literal {
            kind: LiteralKind::Integer,
            lexeme: &source[0..2],
        };
        match e_lit {
            ExpressionKind::Literal { kind: _, lexeme } => assert_eq!(lexeme, "42"),
            _ => panic!("expected Literal"),
        }

        let e_id = ExpressionKind::Identifier(Identifier {
            lexeme: &source[3..4],
            span: SourceSpan { start: 3, end: 4 },
        });
        match e_id {
            ExpressionKind::Identifier(id) => assert_eq!(id.lexeme, "x"),
            _ => panic!("expected Identifier"),
        }

        let e_un = ExpressionKind::Unary {
            operator: UnaryOperator::Negate,
            operand: Box::new(Expression {
                kind: ExpressionKind::Identifier(Identifier {
                    lexeme: &source[6..7],
                    span: SourceSpan { start: 6, end: 7 },
                }),
                span: SourceSpan { start: 6, end: 7 },
            }),
        };
        match e_un {
            ExpressionKind::Unary { operator, operand } => {
                match operator {
                    UnaryOperator::Negate => {}
                    _ => panic!("expected Negate"),
                }
                assert_eq!(operand.span.start, 6);
            }
            _ => panic!("expected Unary"),
        }

        let e_bin = ExpressionKind::Binary {
            left: Box::new(Expression {
                kind: ExpressionKind::Identifier(Identifier {
                    lexeme: &source[8..9],
                    span: SourceSpan { start: 8, end: 9 },
                }),
                span: SourceSpan { start: 8, end: 9 },
            }),
            operator: BinaryOperator::Add,
            right: Box::new(Expression {
                kind: ExpressionKind::Identifier(Identifier {
                    lexeme: &source[12..13],
                    span: SourceSpan { start: 12, end: 13 },
                }),
                span: SourceSpan { start: 12, end: 13 },
            }),
        };
        match e_bin {
            ExpressionKind::Binary {
                left: _,
                operator,
                right: _,
            } => match operator {
                BinaryOperator::Add => {}
                _ => panic!("expected Add"),
            },
            _ => panic!("expected Binary"),
        }

        let e_fa = ExpressionKind::FieldAccess {
            receiver: Box::new(Expression {
                kind: ExpressionKind::Identifier(Identifier {
                    lexeme: &source[14..15],
                    span: SourceSpan { start: 14, end: 15 },
                }),
                span: SourceSpan { start: 14, end: 15 },
            }),
            field: Identifier {
                lexeme: &source[16..17],
                span: SourceSpan { start: 16, end: 17 },
            },
        };
        match e_fa {
            ExpressionKind::FieldAccess { receiver: _, field } => {
                assert_eq!(field.lexeme, "f");
            }
            _ => panic!("expected FieldAccess"),
        }

        let e_fc = ExpressionKind::FunctionCall(FunctionCall {
            callee: Identifier {
                lexeme: &source[18..22],
                span: SourceSpan { start: 18, end: 22 },
            },
            arguments: alloc::vec![Expression {
                kind: ExpressionKind::Identifier(Identifier {
                    lexeme: &source[23..24],
                    span: SourceSpan { start: 23, end: 24 },
                }),
                span: SourceSpan { start: 23, end: 24 },
            }],
        });
        match e_fc {
            ExpressionKind::FunctionCall(fc) => {
                assert_eq!(fc.callee.lexeme, "call");
                assert_eq!(fc.arguments.len(), 1);
            }
            _ => panic!("expected FunctionCall"),
        }

        let e_sc = ExpressionKind::StructConstruction {
            type_name: Identifier {
                lexeme: &source[26..30],
                span: SourceSpan { start: 26, end: 30 },
            },
            fields: alloc::vec![FieldInitializer {
                name: Identifier {
                    lexeme: &source[31..32],
                    span: SourceSpan { start: 31, end: 32 },
                },
                value: Expression {
                    kind: ExpressionKind::Literal {
                        kind: LiteralKind::Integer,
                        lexeme: &source[34..35],
                    },
                    span: SourceSpan { start: 34, end: 35 },
                },
            }],
        };
        match e_sc {
            ExpressionKind::StructConstruction { type_name, fields } => {
                assert_eq!(type_name.lexeme, "User");
                assert_eq!(fields.len(), 1);
            }
            _ => panic!("expected StructConstruction"),
        }

        let e_ec = ExpressionKind::EnumConstruction(EnumConstruction::Simple {
            variant: QualifiedName {
                qualifier: Identifier {
                    lexeme: &source[37..38],
                    span: SourceSpan { start: 37, end: 38 },
                },
                name: Identifier {
                    lexeme: &source[40..41],
                    span: SourceSpan { start: 40, end: 41 },
                },
            },
        });
        match e_ec {
            ExpressionKind::EnumConstruction(_) => {}
            _ => panic!("expected EnumConstruction"),
        }

        let e_pip = ExpressionKind::Pipeline(Pipeline {
            source: Box::new(Expression {
                kind: ExpressionKind::Identifier(Identifier {
                    lexeme: &source[42..43],
                    span: SourceSpan { start: 42, end: 43 },
                }),
                span: SourceSpan { start: 42, end: 43 },
            }),
            stages: alloc::vec![PipelineStage {
                callee: Identifier {
                    lexeme: &source[47..48],
                    span: SourceSpan { start: 47, end: 48 },
                },
                additional_arguments: alloc::vec![],
            }],
        });
        match e_pip {
            ExpressionKind::Pipeline(pip) => {
                assert_eq!(pip.stages.len(), 1);
                assert_eq!(pip.stages[0].callee.lexeme, "f");
            }
            _ => panic!("expected Pipeline"),
        }

        let e_when = ExpressionKind::When(WhenExpression {
            subject: Box::new(Expression {
                kind: ExpressionKind::Identifier(Identifier {
                    lexeme: &source[42..43],
                    span: SourceSpan { start: 42, end: 43 },
                }),
                span: SourceSpan { start: 42, end: 43 },
            }),
            correspondences: alloc::vec![WhenCorrespondence {
                pattern: WhenPattern::Simple {
                    variant: QualifiedName {
                        qualifier: Identifier {
                            lexeme: &source[49..53],
                            span: SourceSpan { start: 49, end: 53 },
                        },
                        name: Identifier {
                            lexeme: &source[55..58],
                            span: SourceSpan { start: 55, end: 58 },
                        },
                    },
                },
                result: Expression {
                    kind: ExpressionKind::Literal {
                        kind: LiteralKind::Integer,
                        lexeme: &source[59..60],
                    },
                    span: SourceSpan { start: 59, end: 60 },
                },
            }],
        });
        match e_when {
            ExpressionKind::When(w) => {
                assert_eq!(w.correspondences.len(), 1);
            }
            _ => panic!("expected When"),
        }
    }

    #[test]
    fn function_call_arguments_order() {
        let source = AllocString::from("callee first second");
        let fc = FunctionCall {
            callee: Identifier {
                lexeme: &source[0..6],
                span: SourceSpan { start: 0, end: 6 },
            },
            arguments: alloc::vec![
                Expression {
                    kind: ExpressionKind::Identifier(Identifier {
                        lexeme: &source[7..12],
                        span: SourceSpan { start: 7, end: 12 },
                    }),
                    span: SourceSpan { start: 7, end: 12 },
                },
                Expression {
                    kind: ExpressionKind::Identifier(Identifier {
                        lexeme: &source[13..19],
                        span: SourceSpan { start: 13, end: 19 },
                    }),
                    span: SourceSpan { start: 13, end: 19 },
                },
            ],
        };

        assert_eq!(fc.arguments.len(), 2);
        match &fc.arguments[0].kind {
            ExpressionKind::Identifier(id) => assert_eq!(id.lexeme, "first"),
            _ => panic!("expected first argument Identifier"),
        }
        match &fc.arguments[1].kind {
            ExpressionKind::Identifier(id) => assert_eq!(id.lexeme, "second"),
            _ => panic!("expected second argument Identifier"),
        }
    }

    #[test]
    fn struct_construction_field_order_and_duplicates() {
        let source = AllocString::from("User a 1 b 2 a 3");
        let sc = ExpressionKind::StructConstruction {
            type_name: Identifier {
                lexeme: &source[0..4],
                span: SourceSpan { start: 0, end: 4 },
            },
            fields: alloc::vec![
                FieldInitializer {
                    name: Identifier {
                        lexeme: &source[5..6],
                        span: SourceSpan { start: 5, end: 6 },
                    },
                    value: Expression {
                        kind: ExpressionKind::Literal {
                            kind: LiteralKind::Integer,
                            lexeme: &source[7..8],
                        },
                        span: SourceSpan { start: 7, end: 8 },
                    },
                },
                FieldInitializer {
                    name: Identifier {
                        lexeme: &source[9..10],
                        span: SourceSpan { start: 9, end: 10 },
                    },
                    value: Expression {
                        kind: ExpressionKind::Literal {
                            kind: LiteralKind::Integer,
                            lexeme: &source[11..12],
                        },
                        span: SourceSpan { start: 11, end: 12 },
                    },
                },
                FieldInitializer {
                    name: Identifier {
                        lexeme: &source[13..14],
                        span: SourceSpan { start: 13, end: 14 },
                    },
                    value: Expression {
                        kind: ExpressionKind::Literal {
                            kind: LiteralKind::Integer,
                            lexeme: &source[15..16],
                        },
                        span: SourceSpan { start: 15, end: 16 },
                    },
                },
            ],
        };

        match sc {
            ExpressionKind::StructConstruction { type_name, fields } => {
                assert_eq!(type_name.lexeme, "User");
                assert_eq!(fields.len(), 3);
                assert_eq!(fields[0].name.lexeme, "a");
                assert_eq!(fields[1].name.lexeme, "b");
                assert_eq!(fields[2].name.lexeme, "a");
            }
            _ => panic!("expected StructConstruction"),
        }
    }

    #[test]
    fn pipeline_stages_and_additional_arguments_order() {
        let source = AllocString::from("val first x y second");
        let pipeline = Pipeline {
            source: Box::new(Expression {
                kind: ExpressionKind::Identifier(Identifier {
                    lexeme: &source[0..3],
                    span: SourceSpan { start: 0, end: 3 },
                }),
                span: SourceSpan { start: 0, end: 3 },
            }),
            stages: alloc::vec![
                PipelineStage {
                    callee: Identifier {
                        lexeme: &source[4..9],
                        span: SourceSpan { start: 4, end: 9 },
                    },
                    additional_arguments: alloc::vec![
                        Expression {
                            kind: ExpressionKind::Identifier(Identifier {
                                lexeme: &source[10..11],
                                span: SourceSpan { start: 10, end: 11 },
                            }),
                            span: SourceSpan { start: 10, end: 11 },
                        },
                        Expression {
                            kind: ExpressionKind::Identifier(Identifier {
                                lexeme: &source[12..13],
                                span: SourceSpan { start: 12, end: 13 },
                            }),
                            span: SourceSpan { start: 12, end: 13 },
                        },
                    ],
                },
                PipelineStage {
                    callee: Identifier {
                        lexeme: &source[14..20],
                        span: SourceSpan { start: 14, end: 20 },
                    },
                    additional_arguments: alloc::vec![],
                },
            ],
        };

        assert_eq!(pipeline.stages.len(), 2);
        assert_eq!(pipeline.stages[0].callee.lexeme, "first");
        assert_eq!(pipeline.stages[1].callee.lexeme, "second");

        assert_eq!(pipeline.stages[0].additional_arguments.len(), 2);
        match &pipeline.stages[0].additional_arguments[0].kind {
            ExpressionKind::Identifier(id) => assert_eq!(id.lexeme, "x"),
            _ => panic!("expected Identifier x"),
        }
        match &pipeline.stages[0].additional_arguments[1].kind {
            ExpressionKind::Identifier(id) => assert_eq!(id.lexeme, "y"),
            _ => panic!("expected Identifier y"),
        }
    }
}
