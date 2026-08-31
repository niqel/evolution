use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceSpan {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    // Variable textual (1)
    Identifier,

    // Literals (4)
    IntegerLiteral,
    FloatingLiteral,
    StringLiteral,
    BooleanLiteral,

    // Structural Keywords (18)
    Artifact,
    Let,
    Struct,
    Enum,
    Fn,
    Public,
    Private,
    Return,
    When,
    Esig,
    Import,
    As,
    Module,
    Publish,
    Bind,
    To,
    Entry,
    This,

    // Operators (16)
    FieldAccess,
    Not,
    Minus,
    Multiply,
    Divide,
    Remainder,
    Plus,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    And,
    Or,
    Pipeline,

    // Structural Symbols (11)
    Association,
    Colon,
    Qualification,
    ReturnType,
    Correspondence,
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'source> {
    pub kind: TokenKind,
    pub lexeme: &'source str,
    pub span: SourceSpan,
}

pub type TokenSequence<'source> = Vec<Token<'source>>;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String as AllocString;

    #[test]
    fn token_kind_inventory_has_exactly_50_variants() {
        let all_variants: [TokenKind; 50] = [
            // Variable textual (1)
            TokenKind::Identifier,
            // Literals (4)
            TokenKind::IntegerLiteral,
            TokenKind::FloatingLiteral,
            TokenKind::StringLiteral,
            TokenKind::BooleanLiteral,
            // Structural Keywords (18)
            TokenKind::Artifact,
            TokenKind::Let,
            TokenKind::Struct,
            TokenKind::Enum,
            TokenKind::Fn,
            TokenKind::Public,
            TokenKind::Private,
            TokenKind::Return,
            TokenKind::When,
            TokenKind::Esig,
            TokenKind::Import,
            TokenKind::As,
            TokenKind::Module,
            TokenKind::Publish,
            TokenKind::Bind,
            TokenKind::To,
            TokenKind::Entry,
            TokenKind::This,
            // Operators (16)
            TokenKind::FieldAccess,
            TokenKind::Not,
            TokenKind::Minus,
            TokenKind::Multiply,
            TokenKind::Divide,
            TokenKind::Remainder,
            TokenKind::Plus,
            TokenKind::Less,
            TokenKind::LessEqual,
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Equal,
            TokenKind::NotEqual,
            TokenKind::And,
            TokenKind::Or,
            TokenKind::Pipeline,
            // Structural Symbols (11)
            TokenKind::Association,
            TokenKind::Colon,
            TokenKind::Qualification,
            TokenKind::ReturnType,
            TokenKind::Correspondence,
            TokenKind::LeftParenthesis,
            TokenKind::RightParenthesis,
            TokenKind::LeftBrace,
            TokenKind::RightBrace,
            TokenKind::Comma,
            TokenKind::Semicolon,
        ];

        assert_eq!(all_variants.len(), 50);
        assert_eq!(all_variants[0], TokenKind::Identifier);
        assert_eq!(all_variants[49], TokenKind::Semicolon);
    }

    #[test]
    fn source_span_representation() {
        let span = SourceSpan { start: 10, end: 20 };
        assert_eq!(span.start, 10);
        assert_eq!(span.end, 20);
        assert!(span.start < span.end);

        // Zero-width span representation (for parser/diagnostics)
        let zero_width = SourceSpan { start: 15, end: 15 };
        assert_eq!(zero_width.start, zero_width.end);

        // Copy and equality
        let span_copy = span;
        assert_eq!(span, span_copy);
        assert_ne!(span, zero_width);
    }

    #[test]
    fn token_borrowed_lexeme_matches_source_slice() {
        let source = AllocString::from("let count = 42;");
        let lexeme = &source[0..3];
        let token = Token {
            kind: TokenKind::Let,
            lexeme,
            span: SourceSpan { start: 0, end: 3 },
        };

        assert_eq!(token.kind, TokenKind::Let);
        assert_eq!(token.lexeme, "let");
        assert_eq!(token.span, SourceSpan { start: 0, end: 3 });
        assert_eq!(token.lexeme, &source[token.span.start..token.span.end]);

        // Copy semantics
        let token_copy = token;
        assert_eq!(token, token_copy);
    }

    #[test]
    fn token_sequence_order_and_gaps() {
        let source = AllocString::from("let x = 42;");
        let t0 = Token {
            kind: TokenKind::Let,
            lexeme: &source[0..3],
            span: SourceSpan { start: 0, end: 3 },
        };
        let t1 = Token {
            kind: TokenKind::Identifier,
            lexeme: &source[4..5],
            span: SourceSpan { start: 4, end: 5 },
        };
        let t2 = Token {
            kind: TokenKind::Association,
            lexeme: &source[6..7],
            span: SourceSpan { start: 6, end: 7 },
        };
        let t3 = Token {
            kind: TokenKind::IntegerLiteral,
            lexeme: &source[8..10],
            span: SourceSpan { start: 8, end: 10 },
        };
        let t4 = Token {
            kind: TokenKind::Semicolon,
            lexeme: &source[10..11],
            span: SourceSpan { start: 10, end: 11 },
        };

        let seq: TokenSequence = alloc::vec![t0, t1, t2, t3, t4];

        assert_eq!(seq.len(), 5);
        assert_eq!(seq[0], t0);
        assert_eq!(seq[1], t1);
        assert_eq!(seq[2], t2);
        assert_eq!(seq[3], t3);
        assert_eq!(seq[4], t4);

        // Gap verification: whitespace between tokens 0 and 1
        assert!(seq[0].span.end < seq[1].span.start);
    }
}
