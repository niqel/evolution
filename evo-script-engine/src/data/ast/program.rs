use alloc::vec::Vec;

use crate::data::ast::foundational::ImportDeclaration;
use crate::data::ast::functions::FunctionDefinition;
use crate::data::ast::local_types::{EnumDefinition, StructDefinition};

pub(crate) struct Program<'source> {
    pub(crate) imports: Vec<ImportDeclaration<'source>>,
    pub(crate) declarations: Vec<Declaration<'source>>,
}

pub(crate) enum Declaration<'source> {
    Struct(StructDefinition<'source>),
    Enum(EnumDefinition<'source>),
    Function(FunctionDefinition<'source>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::ast::expressions::{Expression, ExpressionKind, LiteralKind};
    use crate::data::ast::foundational::{Identifier, QualifiedName, Visibility};
    use crate::data::ast::functions::FunctionBody;
    use crate::data::ast::local_types::EnumVariant;
    use crate::data::lexical::SourceSpan;
    use alloc::string::String as AllocString;

    #[test]
    fn program_imports_and_declarations() {
        let source = AllocString::from("Math::Sum Math::Mul User Status Active run 0");
        let program = Program {
            imports: alloc::vec![
                ImportDeclaration {
                    symbol: QualifiedName {
                        qualifier: Identifier {
                            lexeme: &source[0..4],
                            span: SourceSpan { start: 0, end: 4 },
                        },
                        name: Identifier {
                            lexeme: &source[6..9],
                            span: SourceSpan { start: 6, end: 9 },
                        },
                    },
                    alias: None,
                },
                ImportDeclaration {
                    symbol: QualifiedName {
                        qualifier: Identifier {
                            lexeme: &source[10..14],
                            span: SourceSpan { start: 10, end: 14 },
                        },
                        name: Identifier {
                            lexeme: &source[16..19],
                            span: SourceSpan { start: 16, end: 19 },
                        },
                    },
                    alias: None,
                },
            ],
            declarations: alloc::vec![
                Declaration::Struct(StructDefinition {
                    name: Identifier {
                        lexeme: &source[20..24],
                        span: SourceSpan { start: 20, end: 24 },
                    },
                    fields: alloc::vec![],
                }),
                Declaration::Enum(EnumDefinition {
                    name: Identifier {
                        lexeme: &source[25..31],
                        span: SourceSpan { start: 25, end: 31 },
                    },
                    variants: alloc::vec![EnumVariant::Simple {
                        name: Identifier {
                            lexeme: &source[32..38],
                            span: SourceSpan { start: 32, end: 38 },
                        },
                    }],
                }),
                Declaration::Function(FunctionDefinition {
                    visibility: Visibility::Public,
                    name: Identifier {
                        lexeme: &source[39..42],
                        span: SourceSpan { start: 39, end: 42 },
                    },
                    parameters: alloc::vec![],
                    result_type: Identifier {
                        lexeme: &source[39..42],
                        span: SourceSpan { start: 39, end: 42 },
                    },
                    satisfaction: None,
                    body: FunctionBody {
                        statements: alloc::vec![],
                        result: Expression {
                            kind: ExpressionKind::Literal {
                                kind: LiteralKind::Integer,
                                lexeme: &source[43..44],
                            },
                            span: SourceSpan { start: 43, end: 44 },
                        },
                    },
                }),
            ],
        };

        assert_eq!(program.imports.len(), 2);
        assert_eq!(program.imports[0].symbol.qualifier.lexeme, "Math");
        assert_eq!(program.imports[0].symbol.name.lexeme, "Sum");
        assert_eq!(program.imports[1].symbol.qualifier.lexeme, "Math");
        assert_eq!(program.imports[1].symbol.name.lexeme, "Mul");

        assert_eq!(program.declarations.len(), 3);
        match &program.declarations[0] {
            Declaration::Struct(s) => assert_eq!(s.name.lexeme, "User"),
            _ => panic!("expected Struct"),
        }
        match &program.declarations[1] {
            Declaration::Enum(e) => {
                assert_eq!(e.name.lexeme, "Status");
                assert_eq!(e.variants.len(), 1);
            }
            _ => panic!("expected Enum"),
        }
        match &program.declarations[2] {
            Declaration::Function(f) => {
                assert_eq!(f.name.lexeme, "run");
                match f.visibility {
                    Visibility::Public => {}
                    _ => panic!("expected Public"),
                }
            }
            _ => panic!("expected Function"),
        }
    }
}
