use crate::data::lexical::SourceSpan;

pub(crate) struct Identifier<'source> {
    pub(crate) lexeme: &'source str,
    pub(crate) span: SourceSpan,
}

pub(crate) struct QualifiedName<'source> {
    pub(crate) qualifier: Identifier<'source>,
    pub(crate) name: Identifier<'source>,
}

pub(crate) enum Visibility {
    Public,
    Private,
}

pub(crate) struct TypedBinding<'source> {
    pub(crate) type_name: Identifier<'source>,
    pub(crate) name: Identifier<'source>,
}

pub(crate) struct ImportDeclaration<'source> {
    pub(crate) symbol: QualifiedName<'source>,
    pub(crate) alias: Option<Identifier<'source>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String as AllocString;

    #[test]
    fn identifier_borrowing_and_span() {
        let source = AllocString::from("customer");
        let identifier = Identifier {
            lexeme: &source[0..8],
            span: SourceSpan { start: 0, end: 8 },
        };

        assert_eq!(identifier.lexeme, &source[0..8]);
        assert_eq!(identifier.lexeme, "customer");
        assert_eq!(identifier.span.start, 0);
        assert_eq!(identifier.span.end, 8);
    }

    #[test]
    fn qualified_name_qualifier_and_name() {
        let source = AllocString::from("math::sum");
        let qn = QualifiedName {
            qualifier: Identifier {
                lexeme: &source[0..4],
                span: SourceSpan { start: 0, end: 4 },
            },
            name: Identifier {
                lexeme: &source[6..9],
                span: SourceSpan { start: 6, end: 9 },
            },
        };

        assert_eq!(qn.qualifier.lexeme, &source[0..4]);
        assert_eq!(qn.qualifier.lexeme, "math");
        assert_eq!(qn.qualifier.span.start, 0);
        assert_eq!(qn.qualifier.span.end, 4);

        assert_eq!(qn.name.lexeme, &source[6..9]);
        assert_eq!(qn.name.lexeme, "sum");
        assert_eq!(qn.name.span.start, 6);
        assert_eq!(qn.name.span.end, 9);
    }

    #[test]
    fn visibility_variants() {
        let v_public = Visibility::Public;
        match v_public {
            Visibility::Public => {}
            Visibility::Private => panic!("expected Visibility::Public"),
        }

        let v_private = Visibility::Private;
        match v_private {
            Visibility::Private => {}
            Visibility::Public => panic!("expected Visibility::Private"),
        }
    }

    #[test]
    fn typed_binding_representation() {
        let source = AllocString::from("int32 count");
        let binding = TypedBinding {
            type_name: Identifier {
                lexeme: &source[0..5],
                span: SourceSpan { start: 0, end: 5 },
            },
            name: Identifier {
                lexeme: &source[6..11],
                span: SourceSpan { start: 6, end: 11 },
            },
        };

        assert_eq!(binding.type_name.lexeme, "int32");
        assert_eq!(binding.type_name.span.start, 0);
        assert_eq!(binding.type_name.span.end, 5);

        assert_eq!(binding.name.lexeme, "count");
        assert_eq!(binding.name.span.start, 6);
        assert_eq!(binding.name.span.end, 11);
    }

    #[test]
    fn import_declaration_without_alias() {
        let source = AllocString::from("import math::sum;");
        let import_decl = ImportDeclaration {
            symbol: QualifiedName {
                qualifier: Identifier {
                    lexeme: &source[7..11],
                    span: SourceSpan { start: 7, end: 11 },
                },
                name: Identifier {
                    lexeme: &source[13..16],
                    span: SourceSpan { start: 13, end: 16 },
                },
            },
            alias: None,
        };

        assert_eq!(import_decl.symbol.qualifier.lexeme, "math");
        assert_eq!(import_decl.symbol.name.lexeme, "sum");
        assert!(import_decl.alias.is_none());
    }

    #[test]
    fn import_declaration_with_alias() {
        let source = AllocString::from("import math::sum as add;");
        let import_decl = ImportDeclaration {
            symbol: QualifiedName {
                qualifier: Identifier {
                    lexeme: &source[7..11],
                    span: SourceSpan { start: 7, end: 11 },
                },
                name: Identifier {
                    lexeme: &source[13..16],
                    span: SourceSpan { start: 13, end: 16 },
                },
            },
            alias: Some(Identifier {
                lexeme: &source[20..23],
                span: SourceSpan { start: 20, end: 23 },
            }),
        };

        assert_eq!(import_decl.symbol.qualifier.lexeme, "math");
        assert_eq!(import_decl.symbol.name.lexeme, "sum");
        match import_decl.alias {
            Some(alias_id) => {
                assert_eq!(alias_id.lexeme, "add");
                assert_eq!(alias_id.span.start, 20);
                assert_eq!(alias_id.span.end, 23);
            }
            None => panic!("expected Some alias"),
        }
    }
}
