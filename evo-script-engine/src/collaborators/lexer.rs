use alloc::vec::Vec;

use crate::data::failures::{CompileFailure, CompileFailureKind, LexicalFailure};
use crate::data::lexical::{SourceSpan, Token, TokenKind, TokenSequence};

pub type Lex = for<'source> fn(&'source str) -> Result<TokenSequence<'source>, CompileFailure>;

pub fn lex_source<'source>(source: &'source str) -> Result<TokenSequence<'source>, CompileFailure> {
    let mut tokens = Vec::new();
    let bytes = source.as_bytes();
    let mut cursor = 0;

    while cursor < bytes.len() {
        let b = bytes[cursor];

        // 1. Whitespace
        if b == b' ' || b == b'\t' || b == b'\n' || b == b'\r' {
            cursor += 1;
            continue;
        }

        // 2. Comments (// until line end)
        if b == b'/' && cursor + 1 < bytes.len() && bytes[cursor + 1] == b'/' {
            cursor += 2;
            while cursor < bytes.len() && bytes[cursor] != b'\n' && bytes[cursor] != b'\r' {
                cursor += 1;
            }
            continue;
        }

        // 3. String literal ("...")
        if b == b'"' {
            let start = cursor;
            cursor += 1;
            loop {
                if cursor >= bytes.len() {
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Lexical(
                            LexicalFailure::UnterminatedStringLiteral,
                        ),
                        source_span: SourceSpan {
                            start,
                            end: bytes.len(),
                        },
                    });
                }
                let sb = bytes[cursor];
                if sb == b'"' {
                    cursor += 1;
                    tokens.push(Token {
                        kind: TokenKind::StringLiteral,
                        lexeme: &source[start..cursor],
                        span: SourceSpan { start, end: cursor },
                    });
                    break;
                } else if sb == b'\n' || sb == b'\r' {
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Lexical(
                            LexicalFailure::PhysicalNewlineInStringLiteral,
                        ),
                        source_span: SourceSpan {
                            start: cursor,
                            end: cursor + 1,
                        },
                    });
                } else if sb == b'\\' {
                    let esc_start = cursor;
                    cursor += 1;
                    if cursor >= bytes.len() {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Lexical(
                                LexicalFailure::UnterminatedStringLiteral,
                            ),
                            source_span: SourceSpan {
                                start,
                                end: bytes.len(),
                            },
                        });
                    }
                    let esc_byte = bytes[cursor];
                    if esc_byte == b'\n' || esc_byte == b'\r' {
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Lexical(
                                LexicalFailure::PhysicalNewlineInStringLiteral,
                            ),
                            source_span: SourceSpan {
                                start: cursor,
                                end: cursor + 1,
                            },
                        });
                    }
                    match esc_byte {
                        b'"' | b'\\' | b'n' | b'r' | b't' => {
                            cursor += 1;
                        }
                        _ => {
                            let ch = source[cursor..].chars().next().unwrap_or(esc_byte as char);
                            let ch_len = ch.len_utf8();
                            return Err(CompileFailure {
                                kind: CompileFailureKind::Lexical(
                                    LexicalFailure::InvalidStringEscape(ch),
                                ),
                                source_span: SourceSpan {
                                    start: esc_start,
                                    end: esc_start + 1 + ch_len,
                                },
                            });
                        }
                    }
                } else {
                    let ch = source[cursor..].chars().next().unwrap();
                    cursor += ch.len_utf8();
                }
            }
            continue;
        }

        // 4. Two-character and multi-character operators / symbols
        if cursor + 1 < bytes.len() {
            let pair = &bytes[cursor..cursor + 2];
            match pair {
                b"::" => {
                    tokens.push(Token {
                        kind: TokenKind::Qualification,
                        lexeme: &source[cursor..cursor + 2],
                        span: SourceSpan {
                            start: cursor,
                            end: cursor + 2,
                        },
                    });
                    cursor += 2;
                    continue;
                }
                b"->" => {
                    tokens.push(Token {
                        kind: TokenKind::ReturnType,
                        lexeme: &source[cursor..cursor + 2],
                        span: SourceSpan {
                            start: cursor,
                            end: cursor + 2,
                        },
                    });
                    cursor += 2;
                    continue;
                }
                b"=>" => {
                    tokens.push(Token {
                        kind: TokenKind::Correspondence,
                        lexeme: &source[cursor..cursor + 2],
                        span: SourceSpan {
                            start: cursor,
                            end: cursor + 2,
                        },
                    });
                    cursor += 2;
                    continue;
                }
                b"==" => {
                    tokens.push(Token {
                        kind: TokenKind::Equal,
                        lexeme: &source[cursor..cursor + 2],
                        span: SourceSpan {
                            start: cursor,
                            end: cursor + 2,
                        },
                    });
                    cursor += 2;
                    continue;
                }
                b"!=" => {
                    tokens.push(Token {
                        kind: TokenKind::NotEqual,
                        lexeme: &source[cursor..cursor + 2],
                        span: SourceSpan {
                            start: cursor,
                            end: cursor + 2,
                        },
                    });
                    cursor += 2;
                    continue;
                }
                b"<=" => {
                    tokens.push(Token {
                        kind: TokenKind::LessEqual,
                        lexeme: &source[cursor..cursor + 2],
                        span: SourceSpan {
                            start: cursor,
                            end: cursor + 2,
                        },
                    });
                    cursor += 2;
                    continue;
                }
                b">=" => {
                    tokens.push(Token {
                        kind: TokenKind::GreaterEqual,
                        lexeme: &source[cursor..cursor + 2],
                        span: SourceSpan {
                            start: cursor,
                            end: cursor + 2,
                        },
                    });
                    cursor += 2;
                    continue;
                }
                b"&&" => {
                    tokens.push(Token {
                        kind: TokenKind::And,
                        lexeme: &source[cursor..cursor + 2],
                        span: SourceSpan {
                            start: cursor,
                            end: cursor + 2,
                        },
                    });
                    cursor += 2;
                    continue;
                }
                b"||" => {
                    tokens.push(Token {
                        kind: TokenKind::Or,
                        lexeme: &source[cursor..cursor + 2],
                        span: SourceSpan {
                            start: cursor,
                            end: cursor + 2,
                        },
                    });
                    cursor += 2;
                    continue;
                }
                b"|>" => {
                    tokens.push(Token {
                        kind: TokenKind::Pipeline,
                        lexeme: &source[cursor..cursor + 2],
                        span: SourceSpan {
                            start: cursor,
                            end: cursor + 2,
                        },
                    });
                    cursor += 2;
                    continue;
                }
                _ => {}
            }
        }

        // 5. Single-character operators / symbols
        match b {
            b':' => {
                tokens.push(Token {
                    kind: TokenKind::Colon,
                    lexeme: &source[cursor..cursor + 1],
                    span: SourceSpan {
                        start: cursor,
                        end: cursor + 1,
                    },
                });
                cursor += 1;
                continue;
            }
            b'-' => {
                tokens.push(Token {
                    kind: TokenKind::Minus,
                    lexeme: &source[cursor..cursor + 1],
                    span: SourceSpan {
                        start: cursor,
                        end: cursor + 1,
                    },
                });
                cursor += 1;
                continue;
            }
            b'=' => {
                tokens.push(Token {
                    kind: TokenKind::Association,
                    lexeme: &source[cursor..cursor + 1],
                    span: SourceSpan {
                        start: cursor,
                        end: cursor + 1,
                    },
                });
                cursor += 1;
                continue;
            }
            b'!' => {
                tokens.push(Token {
                    kind: TokenKind::Not,
                    lexeme: &source[cursor..cursor + 1],
                    span: SourceSpan {
                        start: cursor,
                        end: cursor + 1,
                    },
                });
                cursor += 1;
                continue;
            }
            b'<' => {
                tokens.push(Token {
                    kind: TokenKind::Less,
                    lexeme: &source[cursor..cursor + 1],
                    span: SourceSpan {
                        start: cursor,
                        end: cursor + 1,
                    },
                });
                cursor += 1;
                continue;
            }
            b'>' => {
                tokens.push(Token {
                    kind: TokenKind::Greater,
                    lexeme: &source[cursor..cursor + 1],
                    span: SourceSpan {
                        start: cursor,
                        end: cursor + 1,
                    },
                });
                cursor += 1;
                continue;
            }
            b'.' => {
                tokens.push(Token {
                    kind: TokenKind::FieldAccess,
                    lexeme: &source[cursor..cursor + 1],
                    span: SourceSpan {
                        start: cursor,
                        end: cursor + 1,
                    },
                });
                cursor += 1;
                continue;
            }
            b'*' => {
                tokens.push(Token {
                    kind: TokenKind::Multiply,
                    lexeme: &source[cursor..cursor + 1],
                    span: SourceSpan {
                        start: cursor,
                        end: cursor + 1,
                    },
                });
                cursor += 1;
                continue;
            }
            b'/' => {
                tokens.push(Token {
                    kind: TokenKind::Divide,
                    lexeme: &source[cursor..cursor + 1],
                    span: SourceSpan {
                        start: cursor,
                        end: cursor + 1,
                    },
                });
                cursor += 1;
                continue;
            }
            b'%' => {
                tokens.push(Token {
                    kind: TokenKind::Remainder,
                    lexeme: &source[cursor..cursor + 1],
                    span: SourceSpan {
                        start: cursor,
                        end: cursor + 1,
                    },
                });
                cursor += 1;
                continue;
            }
            b'+' => {
                tokens.push(Token {
                    kind: TokenKind::Plus,
                    lexeme: &source[cursor..cursor + 1],
                    span: SourceSpan {
                        start: cursor,
                        end: cursor + 1,
                    },
                });
                cursor += 1;
                continue;
            }
            b'(' => {
                tokens.push(Token {
                    kind: TokenKind::LeftParenthesis,
                    lexeme: &source[cursor..cursor + 1],
                    span: SourceSpan {
                        start: cursor,
                        end: cursor + 1,
                    },
                });
                cursor += 1;
                continue;
            }
            b')' => {
                tokens.push(Token {
                    kind: TokenKind::RightParenthesis,
                    lexeme: &source[cursor..cursor + 1],
                    span: SourceSpan {
                        start: cursor,
                        end: cursor + 1,
                    },
                });
                cursor += 1;
                continue;
            }
            b'{' => {
                tokens.push(Token {
                    kind: TokenKind::LeftBrace,
                    lexeme: &source[cursor..cursor + 1],
                    span: SourceSpan {
                        start: cursor,
                        end: cursor + 1,
                    },
                });
                cursor += 1;
                continue;
            }
            b'}' => {
                tokens.push(Token {
                    kind: TokenKind::RightBrace,
                    lexeme: &source[cursor..cursor + 1],
                    span: SourceSpan {
                        start: cursor,
                        end: cursor + 1,
                    },
                });
                cursor += 1;
                continue;
            }
            b',' => {
                tokens.push(Token {
                    kind: TokenKind::Comma,
                    lexeme: &source[cursor..cursor + 1],
                    span: SourceSpan {
                        start: cursor,
                        end: cursor + 1,
                    },
                });
                cursor += 1;
                continue;
            }
            b';' => {
                tokens.push(Token {
                    kind: TokenKind::Semicolon,
                    lexeme: &source[cursor..cursor + 1],
                    span: SourceSpan {
                        start: cursor,
                        end: cursor + 1,
                    },
                });
                cursor += 1;
                continue;
            }
            _ => {}
        }

        // 6. Invalid identifier starting with '_'
        if b == b'_' {
            let start = cursor;
            while cursor < bytes.len()
                && (bytes[cursor].is_ascii_alphanumeric() || bytes[cursor] == b'_')
            {
                cursor += 1;
            }
            return Err(CompileFailure {
                kind: CompileFailureKind::Lexical(LexicalFailure::InvalidIdentifier),
                source_span: SourceSpan { start, end: cursor },
            });
        }

        // 7. Numbers (IntegerLiteral, FloatingLiteral, or MalformedNumericLiteral / InvalidIdentifier)
        if b.is_ascii_digit() {
            let start = cursor;
            while cursor < bytes.len() && bytes[cursor].is_ascii_digit() {
                cursor += 1;
            }

            // Check if followed by '_' (e.g. 1_000)
            if cursor < bytes.len() && bytes[cursor] == b'_' {
                while cursor < bytes.len()
                    && (bytes[cursor].is_ascii_alphanumeric()
                        || bytes[cursor] == b'_'
                        || bytes[cursor] == b'.')
                {
                    cursor += 1;
                }
                return Err(CompileFailure {
                    kind: CompileFailureKind::Lexical(LexicalFailure::MalformedNumericLiteral),
                    source_span: SourceSpan { start, end: cursor },
                });
            }

            // Check if followed by letters other than 'e' / 'E' (e.g. 2worker)
            if cursor < bytes.len()
                && (bytes[cursor].is_ascii_alphabetic()
                    && bytes[cursor] != b'e'
                    && bytes[cursor] != b'E')
            {
                while cursor < bytes.len()
                    && (bytes[cursor].is_ascii_alphanumeric() || bytes[cursor] == b'_')
                {
                    cursor += 1;
                }
                return Err(CompileFailure {
                    kind: CompileFailureKind::Lexical(LexicalFailure::InvalidIdentifier),
                    source_span: SourceSpan { start, end: cursor },
                });
            }

            let mut is_float = false;

            // Check decimal point: '.' followed by digit
            if cursor < bytes.len() && bytes[cursor] == b'.' {
                if cursor + 1 < bytes.len() && bytes[cursor + 1].is_ascii_digit() {
                    is_float = true;
                    cursor += 1; // consume '.'
                    while cursor < bytes.len() && bytes[cursor].is_ascii_digit() {
                        cursor += 1;
                    }
                    if cursor < bytes.len() && bytes[cursor] == b'_' {
                        while cursor < bytes.len()
                            && (bytes[cursor].is_ascii_alphanumeric()
                                || bytes[cursor] == b'_'
                                || bytes[cursor] == b'.')
                        {
                            cursor += 1;
                        }
                        return Err(CompileFailure {
                            kind: CompileFailureKind::Lexical(
                                LexicalFailure::MalformedNumericLiteral,
                            ),
                            source_span: SourceSpan { start, end: cursor },
                        });
                    }
                }
            }

            // Check scientific exponent: 'e' or 'E'
            if cursor < bytes.len() && (bytes[cursor] == b'e' || bytes[cursor] == b'E') {
                is_float = true;
                cursor += 1; // consume 'e' or 'E'
                if cursor < bytes.len() && (bytes[cursor] == b'+' || bytes[cursor] == b'-') {
                    cursor += 1; // consume '+' or '-'
                }
                let exp_digits_start = cursor;
                while cursor < bytes.len() && bytes[cursor].is_ascii_digit() {
                    cursor += 1;
                }
                if cursor == exp_digits_start {
                    // Incomplete exponent (e.g. 1e, 1e+, 1.5e, etc.)
                    while cursor < bytes.len()
                        && (bytes[cursor].is_ascii_alphanumeric() || bytes[cursor] == b'_')
                    {
                        cursor += 1;
                    }
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Lexical(LexicalFailure::MalformedNumericLiteral),
                        source_span: SourceSpan { start, end: cursor },
                    });
                }
                if cursor < bytes.len() && bytes[cursor] == b'_' {
                    while cursor < bytes.len()
                        && (bytes[cursor].is_ascii_alphanumeric()
                            || bytes[cursor] == b'_'
                            || bytes[cursor] == b'.')
                    {
                        cursor += 1;
                    }
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Lexical(LexicalFailure::MalformedNumericLiteral),
                        source_span: SourceSpan { start, end: cursor },
                    });
                }
            }

            // Check if trailing identifier characters attached
            if cursor < bytes.len()
                && (bytes[cursor].is_ascii_alphabetic() || bytes[cursor] == b'_')
            {
                while cursor < bytes.len()
                    && (bytes[cursor].is_ascii_alphanumeric() || bytes[cursor] == b'_')
                {
                    cursor += 1;
                }
                return Err(CompileFailure {
                    kind: CompileFailureKind::Lexical(LexicalFailure::MalformedNumericLiteral),
                    source_span: SourceSpan { start, end: cursor },
                });
            }

            let lexeme = &source[start..cursor];
            let kind = if is_float {
                TokenKind::FloatingLiteral
            } else {
                TokenKind::IntegerLiteral
            };
            tokens.push(Token {
                kind,
                lexeme,
                span: SourceSpan { start, end: cursor },
            });
            continue;
        }

        // 8. Identifiers and Keywords
        if b.is_ascii_alphabetic() {
            let start = cursor;
            while cursor < bytes.len()
                && (bytes[cursor].is_ascii_alphanumeric() || bytes[cursor] == b'_')
            {
                cursor += 1;
            }

            // Check if followed by a non-ASCII character (e.g. niño, México)
            if cursor < bytes.len() {
                let next_ch = source[cursor..].chars().next().unwrap();
                if next_ch.is_alphabetic() && !next_ch.is_ascii() {
                    while cursor < bytes.len() {
                        let ch = source[cursor..].chars().next().unwrap();
                        if ch.is_alphanumeric() || ch == '_' {
                            cursor += ch.len_utf8();
                        } else {
                            break;
                        }
                    }
                    return Err(CompileFailure {
                        kind: CompileFailureKind::Lexical(LexicalFailure::InvalidIdentifier),
                        source_span: SourceSpan { start, end: cursor },
                    });
                }
            }

            let lexeme = &source[start..cursor];
            let kind = match lexeme {
                "artifact" => TokenKind::Artifact,
                "let" => TokenKind::Let,
                "struct" => TokenKind::Struct,
                "enum" => TokenKind::Enum,
                "fn" => TokenKind::Fn,
                "public" => TokenKind::Public,
                "private" => TokenKind::Private,
                "return" => TokenKind::Return,
                "when" => TokenKind::When,
                "esig" => TokenKind::Esig,
                "import" => TokenKind::Import,
                "as" => TokenKind::As,
                "module" => TokenKind::Module,
                "publish" => TokenKind::Publish,
                "bind" => TokenKind::Bind,
                "to" => TokenKind::To,
                "entry" => TokenKind::Entry,
                "this" => TokenKind::This,
                "true" => TokenKind::BooleanLiteral,
                "false" => TokenKind::BooleanLiteral,
                _ => TokenKind::Identifier,
            };

            tokens.push(Token {
                kind,
                lexeme,
                span: SourceSpan { start, end: cursor },
            });
            continue;
        }

        // 9. Non-ASCII letter (e.g. χρήστης, 日本, etc.)
        let ch = source[cursor..].chars().next().unwrap();
        if ch.is_alphabetic() {
            let start = cursor;
            while cursor < bytes.len() {
                let next_ch = source[cursor..].chars().next().unwrap();
                if next_ch.is_alphanumeric() || next_ch == '_' {
                    cursor += next_ch.len_utf8();
                } else {
                    break;
                }
            }
            return Err(CompileFailure {
                kind: CompileFailureKind::Lexical(LexicalFailure::InvalidIdentifier),
                source_span: SourceSpan { start, end: cursor },
            });
        }

        // 10. Unrecognized character
        let start = cursor;
        let ch_len = ch.len_utf8();
        cursor += ch_len;
        return Err(CompileFailure {
            kind: CompileFailureKind::Lexical(LexicalFailure::UnrecognizedCharacter(ch)),
            source_span: SourceSpan {
                start,
                end: start + ch_len,
            },
        });
    }

    Ok(tokens)
}

pub const LEX_SOURCE: Lex = lex_source;

#[cfg(test)]
mod tests {
    use super::*;

    fn unwrap_ok<'source>(
        res: Result<TokenSequence<'source>, CompileFailure>,
    ) -> TokenSequence<'source> {
        match res {
            Ok(tokens) => tokens,
            Err(_) => panic!("expected Ok"),
        }
    }

    fn unwrap_err<'source>(res: Result<TokenSequence<'source>, CompileFailure>) -> CompileFailure {
        match res {
            Ok(_) => panic!("expected Err"),
            Err(err) => err,
        }
    }

    #[test]
    fn lex_source_binding_and_type_check() {
        let lex: Lex = lex_source;
        let tokens = unwrap_ok(lex("let x = 1;"));
        assert_eq!(tokens.len(), 5);

        let bound_lex: Lex = LEX_SOURCE;
        let bound_tokens = unwrap_ok(bound_lex("let x = 1;"));
        assert_eq!(bound_tokens.len(), 5);
    }

    #[test]
    fn lex_all_50_token_kinds() {
        let source = r#"
            identifier_name
            12345
            123.456
            "hello"
            true false
            artifact let struct enum fn public private return when esig import as module publish bind to entry this
            . ! - * / % + < <= > >= == != && || |>
            = : :: -> => ( ) { } , ;
        "#;

        let tokens = unwrap_ok(lex_source(source));
        let kinds: Vec<TokenKind> = tokens.iter().map(|t| t.kind).collect();

        // 1. Identifier (1)
        assert!(kinds.contains(&TokenKind::Identifier));

        // 2. Literals (4)
        assert!(kinds.contains(&TokenKind::IntegerLiteral));
        assert!(kinds.contains(&TokenKind::FloatingLiteral));
        assert!(kinds.contains(&TokenKind::StringLiteral));
        assert!(kinds.contains(&TokenKind::BooleanLiteral));

        // 3. Structural Keywords (18)
        assert!(kinds.contains(&TokenKind::Artifact));
        assert!(kinds.contains(&TokenKind::Let));
        assert!(kinds.contains(&TokenKind::Struct));
        assert!(kinds.contains(&TokenKind::Enum));
        assert!(kinds.contains(&TokenKind::Fn));
        assert!(kinds.contains(&TokenKind::Public));
        assert!(kinds.contains(&TokenKind::Private));
        assert!(kinds.contains(&TokenKind::Return));
        assert!(kinds.contains(&TokenKind::When));
        assert!(kinds.contains(&TokenKind::Esig));
        assert!(kinds.contains(&TokenKind::Import));
        assert!(kinds.contains(&TokenKind::As));
        assert!(kinds.contains(&TokenKind::Module));
        assert!(kinds.contains(&TokenKind::Publish));
        assert!(kinds.contains(&TokenKind::Bind));
        assert!(kinds.contains(&TokenKind::To));
        assert!(kinds.contains(&TokenKind::Entry));
        assert!(kinds.contains(&TokenKind::This));

        // 4. Operators (16)
        assert!(kinds.contains(&TokenKind::FieldAccess));
        assert!(kinds.contains(&TokenKind::Not));
        assert!(kinds.contains(&TokenKind::Minus));
        assert!(kinds.contains(&TokenKind::Multiply));
        assert!(kinds.contains(&TokenKind::Divide));
        assert!(kinds.contains(&TokenKind::Remainder));
        assert!(kinds.contains(&TokenKind::Plus));
        assert!(kinds.contains(&TokenKind::Less));
        assert!(kinds.contains(&TokenKind::LessEqual));
        assert!(kinds.contains(&TokenKind::Greater));
        assert!(kinds.contains(&TokenKind::GreaterEqual));
        assert!(kinds.contains(&TokenKind::Equal));
        assert!(kinds.contains(&TokenKind::NotEqual));
        assert!(kinds.contains(&TokenKind::And));
        assert!(kinds.contains(&TokenKind::Or));
        assert!(kinds.contains(&TokenKind::Pipeline));

        // 5. Structural Symbols (11)
        assert!(kinds.contains(&TokenKind::Association));
        assert!(kinds.contains(&TokenKind::Colon));
        assert!(kinds.contains(&TokenKind::Qualification));
        assert!(kinds.contains(&TokenKind::ReturnType));
        assert!(kinds.contains(&TokenKind::Correspondence));
        assert!(kinds.contains(&TokenKind::LeftParenthesis));
        assert!(kinds.contains(&TokenKind::RightParenthesis));
        assert!(kinds.contains(&TokenKind::LeftBrace));
        assert!(kinds.contains(&TokenKind::RightBrace));
        assert!(kinds.contains(&TokenKind::Comma));
        assert!(kinds.contains(&TokenKind::Semicolon));
    }

    #[test]
    fn lex_keywords_vs_identifiers_and_booleans() {
        let source = "use scope filter return_value true_value int int32 float64 true false True False TRUE FALSE";
        let tokens = unwrap_ok(lex_source(source));

        assert_eq!(tokens[0].kind, TokenKind::Identifier);
        assert_eq!(tokens[0].lexeme, "use");

        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[1].lexeme, "scope");

        assert_eq!(tokens[2].kind, TokenKind::Identifier);
        assert_eq!(tokens[2].lexeme, "filter");

        assert_eq!(tokens[3].kind, TokenKind::Identifier);
        assert_eq!(tokens[3].lexeme, "return_value");

        assert_eq!(tokens[4].kind, TokenKind::Identifier);
        assert_eq!(tokens[4].lexeme, "true_value");

        assert_eq!(tokens[5].kind, TokenKind::Identifier);
        assert_eq!(tokens[5].lexeme, "int");

        assert_eq!(tokens[6].kind, TokenKind::Identifier);
        assert_eq!(tokens[6].lexeme, "int32");

        assert_eq!(tokens[7].kind, TokenKind::Identifier);
        assert_eq!(tokens[7].lexeme, "float64");

        assert_eq!(tokens[8].kind, TokenKind::BooleanLiteral);
        assert_eq!(tokens[8].lexeme, "true");

        assert_eq!(tokens[9].kind, TokenKind::BooleanLiteral);
        assert_eq!(tokens[9].lexeme, "false");

        assert_eq!(tokens[10].kind, TokenKind::Identifier);
        assert_eq!(tokens[10].lexeme, "True");

        assert_eq!(tokens[11].kind, TokenKind::Identifier);
        assert_eq!(tokens[11].lexeme, "False");

        assert_eq!(tokens[12].kind, TokenKind::Identifier);
        assert_eq!(tokens[12].lexeme, "TRUE");

        assert_eq!(tokens[13].kind, TokenKind::Identifier);
        assert_eq!(tokens[13].lexeme, "FALSE");
    }

    #[test]
    fn lex_whitespace_and_comments() {
        let empty_source = "   \t \r\n \n \r ";
        let tokens = unwrap_ok(lex_source(empty_source));
        assert_eq!(tokens.len(), 0);

        let comment_source = "
            // this is a comment
            let x = 10; // trailing comment
            // another full comment line
        ";
        let tokens = unwrap_ok(lex_source(comment_source));
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[2].kind, TokenKind::Association);
        assert_eq!(tokens[3].kind, TokenKind::IntegerLiteral);
        assert_eq!(tokens[4].kind, TokenKind::Semicolon);

        let url_string = "\"https://example.com/path\"";
        let url_tokens = unwrap_ok(lex_source(url_string));
        assert_eq!(url_tokens.len(), 1);
        assert_eq!(url_tokens[0].kind, TokenKind::StringLiteral);
        assert_eq!(url_tokens[0].lexeme, "\"https://example.com/path\"");
    }

    #[test]
    fn lex_longest_match_operators_and_symbols() {
        let source = ":: : -> - = == => ! != < <= > >= && || |> /";
        let tokens = unwrap_ok(lex_source(source));

        assert_eq!(tokens[0].kind, TokenKind::Qualification);
        assert_eq!(tokens[0].lexeme, "::");

        assert_eq!(tokens[1].kind, TokenKind::Colon);
        assert_eq!(tokens[1].lexeme, ":");

        assert_eq!(tokens[2].kind, TokenKind::ReturnType);
        assert_eq!(tokens[2].lexeme, "->");

        assert_eq!(tokens[3].kind, TokenKind::Minus);
        assert_eq!(tokens[3].lexeme, "-");

        assert_eq!(tokens[4].kind, TokenKind::Association);
        assert_eq!(tokens[4].lexeme, "=");

        assert_eq!(tokens[5].kind, TokenKind::Equal);
        assert_eq!(tokens[5].lexeme, "==");

        assert_eq!(tokens[6].kind, TokenKind::Correspondence);
        assert_eq!(tokens[6].lexeme, "=>");

        assert_eq!(tokens[7].kind, TokenKind::Not);
        assert_eq!(tokens[7].lexeme, "!");

        assert_eq!(tokens[8].kind, TokenKind::NotEqual);
        assert_eq!(tokens[8].lexeme, "!=");

        assert_eq!(tokens[9].kind, TokenKind::Less);
        assert_eq!(tokens[9].lexeme, "<");

        assert_eq!(tokens[10].kind, TokenKind::LessEqual);
        assert_eq!(tokens[10].lexeme, "<=");

        assert_eq!(tokens[11].kind, TokenKind::Greater);
        assert_eq!(tokens[11].lexeme, ">");

        assert_eq!(tokens[12].kind, TokenKind::GreaterEqual);
        assert_eq!(tokens[12].lexeme, ">=");

        assert_eq!(tokens[13].kind, TokenKind::And);
        assert_eq!(tokens[13].lexeme, "&&");

        assert_eq!(tokens[14].kind, TokenKind::Or);
        assert_eq!(tokens[14].lexeme, "||");

        assert_eq!(tokens[15].kind, TokenKind::Pipeline);
        assert_eq!(tokens[15].lexeme, "|>");

        assert_eq!(tokens[16].kind, TokenKind::Divide);
        assert_eq!(tokens[16].lexeme, "/");
    }

    #[test]
    fn lex_identifiers_valid_and_failures() {
        let valid_source = "worker Worker worker2 worker_id SearchResult";
        let tokens = unwrap_ok(lex_source(valid_source));
        assert_eq!(tokens.len(), 5);
        for t in tokens {
            assert_eq!(t.kind, TokenKind::Identifier);
        }

        // Invalid: starts with _
        let err1 = unwrap_err(lex_source("_worker"));
        match err1.kind {
            CompileFailureKind::Lexical(LexicalFailure::InvalidIdentifier) => {}
            _ => panic!("expected InvalidIdentifier for _worker"),
        }

        let err2 = unwrap_err(lex_source("_"));
        match err2.kind {
            CompileFailureKind::Lexical(LexicalFailure::InvalidIdentifier) => {}
            _ => panic!("expected InvalidIdentifier for _"),
        }

        // Invalid: starts with digit
        let err3 = unwrap_err(lex_source("2worker"));
        match err3.kind {
            CompileFailureKind::Lexical(LexicalFailure::InvalidIdentifier) => {}
            _ => panic!("expected InvalidIdentifier for 2worker"),
        }

        // Invalid: contains Unicode letters
        let err4 = unwrap_err(lex_source("niño"));
        match err4.kind {
            CompileFailureKind::Lexical(LexicalFailure::InvalidIdentifier) => {}
            _ => panic!("expected InvalidIdentifier for niño"),
        }

        let err5 = unwrap_err(lex_source("México"));
        match err5.kind {
            CompileFailureKind::Lexical(LexicalFailure::InvalidIdentifier) => {}
            _ => panic!("expected InvalidIdentifier for México"),
        }

        let err6 = unwrap_err(lex_source("χρήστης"));
        match err6.kind {
            CompileFailureKind::Lexical(LexicalFailure::InvalidIdentifier) => {}
            _ => panic!("expected InvalidIdentifier for χρήστης"),
        }
    }

    #[test]
    fn lex_numerics_valid_and_minus_separate() {
        let source = "0 123 0.0 10.5 1e10 1E10 1e+10 1e-10 1.5e10 1.5E-10";
        let tokens = unwrap_ok(lex_source(source));
        assert_eq!(tokens[0].kind, TokenKind::IntegerLiteral);
        assert_eq!(tokens[0].lexeme, "0");

        assert_eq!(tokens[1].kind, TokenKind::IntegerLiteral);
        assert_eq!(tokens[1].lexeme, "123");

        assert_eq!(tokens[2].kind, TokenKind::FloatingLiteral);
        assert_eq!(tokens[2].lexeme, "0.0");

        assert_eq!(tokens[3].kind, TokenKind::FloatingLiteral);
        assert_eq!(tokens[3].lexeme, "10.5");

        assert_eq!(tokens[4].kind, TokenKind::FloatingLiteral);
        assert_eq!(tokens[4].lexeme, "1e10");

        assert_eq!(tokens[5].kind, TokenKind::FloatingLiteral);
        assert_eq!(tokens[5].lexeme, "1E10");

        assert_eq!(tokens[6].kind, TokenKind::FloatingLiteral);
        assert_eq!(tokens[6].lexeme, "1e+10");

        assert_eq!(tokens[7].kind, TokenKind::FloatingLiteral);
        assert_eq!(tokens[7].lexeme, "1e-10");

        assert_eq!(tokens[8].kind, TokenKind::FloatingLiteral);
        assert_eq!(tokens[8].lexeme, "1.5e10");

        assert_eq!(tokens[9].kind, TokenKind::FloatingLiteral);
        assert_eq!(tokens[9].lexeme, "1.5E-10");

        // Minus separate
        let minus_nums = "-10 -1.5e10";
        let m_tokens = unwrap_ok(lex_source(minus_nums));
        assert_eq!(m_tokens.len(), 4);
        assert_eq!(m_tokens[0].kind, TokenKind::Minus);
        assert_eq!(m_tokens[1].kind, TokenKind::IntegerLiteral);
        assert_eq!(m_tokens[1].lexeme, "10");
        assert_eq!(m_tokens[2].kind, TokenKind::Minus);
        assert_eq!(m_tokens[3].kind, TokenKind::FloatingLiteral);
        assert_eq!(m_tokens[3].lexeme, "1.5e10");
    }

    #[test]
    fn lex_numerics_malformed_failures() {
        let cases = [
            "1e", "1E", "1e+", "1e-", "1.5e", "1.5e+", "1.5e-", "1_000", "1_000.25", "1e1_000",
        ];
        for case in cases {
            let err = unwrap_err(lex_source(case));
            match err.kind {
                CompileFailureKind::Lexical(LexicalFailure::MalformedNumericLiteral) => {}
                _ => panic!("expected MalformedNumericLiteral for {}", case),
            }
        }
    }

    #[test]
    fn lex_strings_valid_and_escapes() {
        let source = r#""" "hello" "México" "你好" "https://example.com" "\" \\ \n \r \t""#;
        let tokens = unwrap_ok(lex_source(source));
        assert_eq!(tokens.len(), 6);
        for t in &tokens {
            assert_eq!(t.kind, TokenKind::StringLiteral);
        }
        assert_eq!(tokens[0].lexeme, r#""""#);
        assert_eq!(tokens[1].lexeme, r#""hello""#);
        assert_eq!(tokens[2].lexeme, r#""México""#);
        assert_eq!(tokens[3].lexeme, r#""你好""#);
        assert_eq!(tokens[4].lexeme, r#""https://example.com""#);
        assert_eq!(tokens[5].lexeme, r#""\" \\ \n \r \t""#);
    }

    #[test]
    fn lex_strings_failures() {
        // Unterminated
        let err1 = unwrap_err(lex_source("\"unterminated"));
        match err1.kind {
            CompileFailureKind::Lexical(LexicalFailure::UnterminatedStringLiteral) => {}
            _ => panic!("expected UnterminatedStringLiteral"),
        }

        // Invalid string escape
        let err2 = unwrap_err(lex_source(r#""invalid \q escape""#));
        match err2.kind {
            CompileFailureKind::Lexical(LexicalFailure::InvalidStringEscape(ch)) => {
                assert_eq!(ch, 'q');
            }
            _ => panic!("expected InvalidStringEscape"),
        }

        // Physical newline in string (LF)
        let err3 = unwrap_err(lex_source("\"hello\nworld\""));
        match err3.kind {
            CompileFailureKind::Lexical(LexicalFailure::PhysicalNewlineInStringLiteral) => {}
            _ => panic!("expected PhysicalNewlineInStringLiteral for LF"),
        }

        // Physical newline in string (CR)
        let err4 = unwrap_err(lex_source("\"hello\rworld\""));
        match err4.kind {
            CompileFailureKind::Lexical(LexicalFailure::PhysicalNewlineInStringLiteral) => {}
            _ => panic!("expected PhysicalNewlineInStringLiteral for CR"),
        }
    }

    #[test]
    fn lex_all_six_lexical_failure_families() {
        // 1. UnrecognizedCharacter
        let err1 = unwrap_err(lex_source("@"));
        match err1.kind {
            CompileFailureKind::Lexical(LexicalFailure::UnrecognizedCharacter(ch)) => {
                assert_eq!(ch, '@');
            }
            _ => panic!("expected UnrecognizedCharacter"),
        }

        // 2. InvalidIdentifier
        let err2 = unwrap_err(lex_source("_var"));
        match err2.kind {
            CompileFailureKind::Lexical(LexicalFailure::InvalidIdentifier) => {}
            _ => panic!("expected InvalidIdentifier"),
        }

        // 3. MalformedNumericLiteral
        let err3 = unwrap_err(lex_source("1e+"));
        match err3.kind {
            CompileFailureKind::Lexical(LexicalFailure::MalformedNumericLiteral) => {}
            _ => panic!("expected MalformedNumericLiteral"),
        }

        // 4. UnterminatedStringLiteral
        let err4 = unwrap_err(lex_source("\"open"));
        match err4.kind {
            CompileFailureKind::Lexical(LexicalFailure::UnterminatedStringLiteral) => {}
            _ => panic!("expected UnterminatedStringLiteral"),
        }

        // 5. InvalidStringEscape
        let err5 = unwrap_err(lex_source(r#""\x""#));
        match err5.kind {
            CompileFailureKind::Lexical(LexicalFailure::InvalidStringEscape(ch)) => {
                assert_eq!(ch, 'x');
            }
            _ => panic!("expected InvalidStringEscape"),
        }

        // 6. PhysicalNewlineInStringLiteral
        let err6 = unwrap_err(lex_source("\"\n\""));
        match err6.kind {
            CompileFailureKind::Lexical(LexicalFailure::PhysicalNewlineInStringLiteral) => {}
            _ => panic!("expected PhysicalNewlineInStringLiteral"),
        }
    }

    #[test]
    fn lex_strings_backslash_followed_by_physical_newline() {
        // Backslash followed by physical LF
        let source_lf = "\"abc\\\ndef\"";
        let err_lf = unwrap_err(lex_source(source_lf));
        match err_lf.kind {
            CompileFailureKind::Lexical(LexicalFailure::PhysicalNewlineInStringLiteral) => {}
            _ => panic!("expected PhysicalNewlineInStringLiteral for backslash + physical LF"),
        }
        // Span points to the physical LF byte (offset 5)
        assert_eq!(err_lf.source_span, SourceSpan { start: 5, end: 6 });

        // Backslash followed by physical CR
        let source_cr = "\"abc\\\rdef\"";
        let err_cr = unwrap_err(lex_source(source_cr));
        match err_cr.kind {
            CompileFailureKind::Lexical(LexicalFailure::PhysicalNewlineInStringLiteral) => {}
            _ => panic!("expected PhysicalNewlineInStringLiteral for backslash + physical CR"),
        }
        // Span points to the physical CR byte (offset 5)
        assert_eq!(err_cr.source_span, SourceSpan { start: 5, end: 6 });

        // Distinguish from valid \n and \r escapes
        let source_escapes = r#""abc\ndef\rghi""#;
        let tokens = unwrap_ok(lex_source(source_escapes));
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::StringLiteral);
        assert_eq!(tokens[0].lexeme, r#""abc\ndef\rghi""#);
    }

    #[test]
    fn lex_empty_source() {
        let tokens = unwrap_ok(lex_source(""));
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn lex_lexeme_span_invariant_with_unicode() {
        let source = "let greeting = \"¡Hola, mundo!\"; // comentario en español\nfn main() -> int32 { return 42; }";
        let tokens = unwrap_ok(lex_source(source));

        for token in &tokens {
            let sliced = &source[token.span.start..token.span.end];
            assert_eq!(token.lexeme, sliced);
            assert!(token.span.start < token.span.end);
        }
    }
}
