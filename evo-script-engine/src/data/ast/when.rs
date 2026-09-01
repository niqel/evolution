use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::data::ast::expressions::Expression;
use crate::data::ast::foundational::{Identifier, QualifiedName, TypedBinding};

pub(crate) struct WhenExpression<'source> {
    pub(crate) subject: Box<Expression<'source>>,
    pub(crate) correspondences: Vec<WhenCorrespondence<'source>>,
}

pub(crate) struct WhenCorrespondence<'source> {
    pub(crate) pattern: WhenPattern<'source>,
    pub(crate) result: Expression<'source>,
}

pub(crate) enum WhenPattern<'source> {
    Simple {
        variant: QualifiedName<'source>,
    },
    Associated {
        variant: QualifiedName<'source>,
        binding: TypedBinding<'source>,
    },
    Structured {
        variant: QualifiedName<'source>,
        fields: Vec<PatternField<'source>>,
    },
}

pub(crate) struct PatternField<'source> {
    pub(crate) field: Identifier<'source>,
    pub(crate) binding: TypedBinding<'source>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::ast::expressions::{ExpressionKind, LiteralKind};
    use crate::data::lexical::SourceSpan;
    use alloc::string::String as AllocString;

    #[test]
    fn when_pattern_variants() {
        let source = AllocString::from(
            "Status::Active Status::Associated(int32 val) Status::Structured{field: string name}",
        );
        let p_simple = WhenPattern::Simple {
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
        match p_simple {
            WhenPattern::Simple { variant } => {
                assert_eq!(variant.qualifier.lexeme, "Status");
                assert_eq!(variant.name.lexeme, "Active");
            }
            _ => panic!("expected Simple"),
        }

        let p_assoc = WhenPattern::Associated {
            variant: QualifiedName {
                qualifier: Identifier {
                    lexeme: &source[15..21],
                    span: SourceSpan { start: 15, end: 21 },
                },
                name: Identifier {
                    lexeme: &source[23..33],
                    span: SourceSpan { start: 23, end: 33 },
                },
            },
            binding: TypedBinding {
                type_name: Identifier {
                    lexeme: &source[34..39],
                    span: SourceSpan { start: 34, end: 39 },
                },
                name: Identifier {
                    lexeme: &source[40..43],
                    span: SourceSpan { start: 40, end: 43 },
                },
            },
        };
        match p_assoc {
            WhenPattern::Associated { variant, binding } => {
                assert_eq!(variant.name.lexeme, "Associated");
                assert_eq!(binding.type_name.lexeme, "int32");
                assert_eq!(binding.name.lexeme, "val");
            }
            _ => panic!("expected Associated"),
        }

        let p_struct = WhenPattern::Structured {
            variant: QualifiedName {
                qualifier: Identifier {
                    lexeme: &source[45..51],
                    span: SourceSpan { start: 45, end: 51 },
                },
                name: Identifier {
                    lexeme: &source[53..63],
                    span: SourceSpan { start: 53, end: 63 },
                },
            },
            fields: alloc::vec![PatternField {
                field: Identifier {
                    lexeme: &source[64..69],
                    span: SourceSpan { start: 64, end: 69 },
                },
                binding: TypedBinding {
                    type_name: Identifier {
                        lexeme: &source[71..77],
                        span: SourceSpan { start: 71, end: 77 },
                    },
                    name: Identifier {
                        lexeme: &source[78..82],
                        span: SourceSpan { start: 78, end: 82 },
                    },
                },
            }],
        };
        match p_struct {
            WhenPattern::Structured { variant, fields } => {
                assert_eq!(variant.name.lexeme, "Structured");
                assert_eq!(fields.len(), 1);
                assert_eq!(fields[0].field.lexeme, "field");
                assert_eq!(fields[0].binding.name.lexeme, "name");
            }
            _ => panic!("expected Structured"),
        }
    }

    #[test]
    fn when_expression_and_correspondences() {
        let source = AllocString::from("status Status::Active 1");
        let when_expr = WhenExpression {
            subject: Box::new(Expression {
                kind: ExpressionKind::Identifier(Identifier {
                    lexeme: &source[0..6],
                    span: SourceSpan { start: 0, end: 6 },
                }),
                span: SourceSpan { start: 0, end: 6 },
            }),
            correspondences: alloc::vec![WhenCorrespondence {
                pattern: WhenPattern::Simple {
                    variant: QualifiedName {
                        qualifier: Identifier {
                            lexeme: &source[7..13],
                            span: SourceSpan { start: 7, end: 13 },
                        },
                        name: Identifier {
                            lexeme: &source[15..21],
                            span: SourceSpan { start: 15, end: 21 },
                        },
                    },
                },
                result: Expression {
                    kind: ExpressionKind::Literal {
                        kind: LiteralKind::Integer,
                        lexeme: &source[22..23],
                    },
                    span: SourceSpan { start: 22, end: 23 },
                },
            }],
        };

        match when_expr.subject.kind {
            ExpressionKind::Identifier(id) => assert_eq!(id.lexeme, "status"),
            _ => panic!("expected Identifier"),
        }
        assert_eq!(when_expr.correspondences.len(), 1);
        match &when_expr.correspondences[0].pattern {
            WhenPattern::Simple { variant } => assert_eq!(variant.name.lexeme, "Active"),
            _ => panic!("expected Simple"),
        }
        match when_expr.correspondences[0].result.kind {
            ExpressionKind::Literal { kind: _, lexeme } => assert_eq!(lexeme, "1"),
            _ => panic!("expected Literal"),
        }
    }
}
