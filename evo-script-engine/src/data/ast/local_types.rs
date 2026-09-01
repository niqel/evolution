use alloc::vec::Vec;

use crate::data::ast::foundational::Identifier;

pub(crate) struct StructDefinition<'source> {
    pub(crate) name: Identifier<'source>,
    pub(crate) fields: Vec<FieldDefinition<'source>>,
}

pub(crate) struct FieldDefinition<'source> {
    pub(crate) type_name: Identifier<'source>,
    pub(crate) name: Identifier<'source>,
}

pub(crate) struct EnumDefinition<'source> {
    pub(crate) name: Identifier<'source>,
    pub(crate) variants: Vec<EnumVariant<'source>>,
}

pub(crate) enum EnumVariant<'source> {
    Simple {
        name: Identifier<'source>,
    },
    Associated {
        name: Identifier<'source>,
        type_name: Identifier<'source>,
    },
    Structured {
        name: Identifier<'source>,
        fields: Vec<FieldDefinition<'source>>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::lexical::SourceSpan;
    use alloc::string::String as AllocString;

    #[test]
    fn struct_definition_fields_and_empty() {
        let source = AllocString::from("struct User { int32 id; string name; }");
        let struct_def = StructDefinition {
            name: Identifier {
                lexeme: &source[7..11],
                span: SourceSpan { start: 7, end: 11 },
            },
            fields: alloc::vec![
                FieldDefinition {
                    type_name: Identifier {
                        lexeme: &source[14..19],
                        span: SourceSpan { start: 14, end: 19 },
                    },
                    name: Identifier {
                        lexeme: &source[20..22],
                        span: SourceSpan { start: 20, end: 22 },
                    },
                },
                FieldDefinition {
                    type_name: Identifier {
                        lexeme: &source[24..30],
                        span: SourceSpan { start: 24, end: 30 },
                    },
                    name: Identifier {
                        lexeme: &source[31..35],
                        span: SourceSpan { start: 31, end: 35 },
                    },
                },
            ],
        };

        assert_eq!(struct_def.name.lexeme, "User");
        assert_eq!(struct_def.fields.len(), 2);
        assert_eq!(struct_def.fields[0].type_name.lexeme, "int32");
        assert_eq!(struct_def.fields[0].name.lexeme, "id");
        assert_eq!(struct_def.fields[1].type_name.lexeme, "string");
        assert_eq!(struct_def.fields[1].name.lexeme, "name");

        // Struct supporting 0 fields
        let empty_struct = StructDefinition {
            name: Identifier {
                lexeme: &source[7..11],
                span: SourceSpan { start: 7, end: 11 },
            },
            fields: alloc::vec![],
        };
        assert_eq!(empty_struct.fields.len(), 0);
    }

    #[test]
    fn field_definition_borrowed() {
        let source = AllocString::from("float64 score");
        let field = FieldDefinition {
            type_name: Identifier {
                lexeme: &source[0..7],
                span: SourceSpan { start: 0, end: 7 },
            },
            name: Identifier {
                lexeme: &source[8..13],
                span: SourceSpan { start: 8, end: 13 },
            },
        };

        assert_eq!(field.type_name.lexeme, "float64");
        assert_eq!(field.type_name.span.start, 0);
        assert_eq!(field.type_name.span.end, 7);

        assert_eq!(field.name.lexeme, "score");
        assert_eq!(field.name.span.start, 8);
        assert_eq!(field.name.span.end, 13);
    }

    #[test]
    fn enum_definition_and_simple_variant() {
        let source = AllocString::from("enum Status { Pending; }");
        let enum_def = EnumDefinition {
            name: Identifier {
                lexeme: &source[5..11],
                span: SourceSpan { start: 5, end: 11 },
            },
            variants: alloc::vec![EnumVariant::Simple {
                name: Identifier {
                    lexeme: &source[14..21],
                    span: SourceSpan { start: 14, end: 21 },
                },
            }],
        };

        assert_eq!(enum_def.name.lexeme, "Status");
        assert_eq!(enum_def.variants.len(), 1);
        match &enum_def.variants[0] {
            EnumVariant::Simple { name } => {
                assert_eq!(name.lexeme, "Pending");
                assert_eq!(name.span.start, 14);
                assert_eq!(name.span.end, 21);
            }
            _ => panic!("expected EnumVariant::Simple"),
        }
    }

    #[test]
    fn enum_variant_associated() {
        let source = AllocString::from("Value(int32)");
        let variant = EnumVariant::Associated {
            name: Identifier {
                lexeme: &source[0..5],
                span: SourceSpan { start: 0, end: 5 },
            },
            type_name: Identifier {
                lexeme: &source[6..11],
                span: SourceSpan { start: 6, end: 11 },
            },
        };

        match variant {
            EnumVariant::Associated { name, type_name } => {
                assert_eq!(name.lexeme, "Value");
                assert_eq!(name.span.start, 0);
                assert_eq!(name.span.end, 5);

                assert_eq!(type_name.lexeme, "int32");
                assert_eq!(type_name.span.start, 6);
                assert_eq!(type_name.span.end, 11);
            }
            _ => panic!("expected EnumVariant::Associated"),
        }
    }

    #[test]
    fn enum_variant_structured_and_empty() {
        let source = AllocString::from("Person { string name; int32 age; }");
        let variant = EnumVariant::Structured {
            name: Identifier {
                lexeme: &source[0..6],
                span: SourceSpan { start: 0, end: 6 },
            },
            fields: alloc::vec![
                FieldDefinition {
                    type_name: Identifier {
                        lexeme: &source[9..15],
                        span: SourceSpan { start: 9, end: 15 },
                    },
                    name: Identifier {
                        lexeme: &source[16..20],
                        span: SourceSpan { start: 16, end: 20 },
                    },
                },
                FieldDefinition {
                    type_name: Identifier {
                        lexeme: &source[22..27],
                        span: SourceSpan { start: 22, end: 27 },
                    },
                    name: Identifier {
                        lexeme: &source[28..31],
                        span: SourceSpan { start: 28, end: 31 },
                    },
                },
            ],
        };

        match variant {
            EnumVariant::Structured { name, fields } => {
                assert_eq!(name.lexeme, "Person");
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].type_name.lexeme, "string");
                assert_eq!(fields[0].name.lexeme, "name");
                assert_eq!(fields[1].type_name.lexeme, "int32");
                assert_eq!(fields[1].name.lexeme, "age");
            }
            _ => panic!("expected EnumVariant::Structured"),
        }

        // Structured variant with 0 fields
        let empty_structured = EnumVariant::Structured {
            name: Identifier {
                lexeme: &source[0..6],
                span: SourceSpan { start: 0, end: 6 },
            },
            fields: alloc::vec![],
        };
        match empty_structured {
            EnumVariant::Structured { name, fields } => {
                assert_eq!(name.lexeme, "Person");
                assert_eq!(fields.len(), 0);
            }
            _ => panic!("expected EnumVariant::Structured"),
        }
    }

    #[test]
    fn duplicates_preserved_syntactically() {
        let struct_source = AllocString::from("User int32 id");
        let struct_with_dups = StructDefinition {
            name: Identifier {
                lexeme: &struct_source[0..4],
                span: SourceSpan { start: 0, end: 4 },
            },
            fields: alloc::vec![
                FieldDefinition {
                    type_name: Identifier {
                        lexeme: &struct_source[5..10],
                        span: SourceSpan { start: 5, end: 10 },
                    },
                    name: Identifier {
                        lexeme: &struct_source[11..13],
                        span: SourceSpan { start: 11, end: 13 },
                    },
                },
                FieldDefinition {
                    type_name: Identifier {
                        lexeme: &struct_source[5..10],
                        span: SourceSpan { start: 5, end: 10 },
                    },
                    name: Identifier {
                        lexeme: &struct_source[11..13],
                        span: SourceSpan { start: 11, end: 13 },
                    },
                },
            ],
        };

        assert_eq!(struct_with_dups.fields.len(), 2);
        assert_eq!(struct_with_dups.fields[0].name.lexeme, "id");
        assert_eq!(struct_with_dups.fields[1].name.lexeme, "id");

        let enum_source = AllocString::from("State Active Active");
        let enum_with_dups = EnumDefinition {
            name: Identifier {
                lexeme: &enum_source[0..5],
                span: SourceSpan { start: 0, end: 5 },
            },
            variants: alloc::vec![
                EnumVariant::Simple {
                    name: Identifier {
                        lexeme: &enum_source[6..12],
                        span: SourceSpan { start: 6, end: 12 },
                    },
                },
                EnumVariant::Simple {
                    name: Identifier {
                        lexeme: &enum_source[13..19],
                        span: SourceSpan { start: 13, end: 19 },
                    },
                },
            ],
        };

        assert_eq!(enum_with_dups.variants.len(), 2);
    }
}
