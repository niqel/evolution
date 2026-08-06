use crate::definitions::domain::entities::token::Token;
use crate::definitions::domain::entities::token_stream::TokenStream;
use crate::definitions::use_cases::tokenize::TokenizeError;

pub fn resolve<'a>(stream: &mut TokenStream<'a>) -> Result<Option<Token<'a>>, TokenizeError> {
    skip_whitespace(stream);

    let start = stream.position();
    let remaining = stream.remaining();

    if remaining.is_empty() {
        return Ok(None);
    }

    if remaining.starts_with('"') {
        return resolve_string(stream, start, remaining);
    }

    if remaining.starts_with("|>") {
        stream.advance_to(start + 2);
        return Ok(Some(Token::PipelineSeparator));
    }

    if remaining.starts_with('|') {
        return Err(TokenizeError::UnexpectedCharacter);
    }

    if remaining.starts_with(':') {
        stream.advance_to(start + ':'.len_utf8());
        return Ok(Some(Token::Colon));
    }

    if remaining.starts_with(',') {
        stream.advance_to(start + ','.len_utf8());
        return Ok(Some(Token::Comma));
    }

    if remaining.starts_with('(') {
        stream.advance_to(start + '('.len_utf8());
        return Ok(Some(Token::LeftParen));
    }

    if remaining.starts_with(')') {
        stream.advance_to(start + ')'.len_utf8());
        return Ok(Some(Token::RightParen));
    }

    if remaining.starts_with('>') {
        stream.advance_to(start + '>'.len_utf8());
        return Ok(Some(Token::Word(
            &stream.input()[start..start + '>'.len_utf8()],
        )));
    }

    if remaining.starts_with('<') {
        stream.advance_to(start + '<'.len_utf8());
        return Ok(Some(Token::Word(
            &stream.input()[start..start + '<'.len_utf8()],
        )));
    }

    resolve_word(stream, start, remaining)
}

fn skip_whitespace(stream: &mut TokenStream<'_>) {
    let start = stream.position();
    let mut position = start;

    for (offset, character) in stream.remaining().char_indices() {
        if character.is_whitespace() {
            position = start + offset + character.len_utf8();
        } else {
            break;
        }
    }

    stream.advance_to(position);
}

fn resolve_word<'a>(
    stream: &mut TokenStream<'a>,
    start: usize,
    remaining: &'a str,
) -> Result<Option<Token<'a>>, TokenizeError> {
    let mut end = start;

    for (offset, character) in remaining.char_indices() {
        if character == '"' {
            return Err(TokenizeError::UnexpectedCharacter);
        }

        if character.is_whitespace()
            || character == '|'
            || character == ','
            || character == '('
            || character == ')'
            || character == '>'
            || character == '<'
            || character == ':'
        {
            break;
        }

        end = start + offset + character.len_utf8();
    }

    if end == start {
        return Err(TokenizeError::UnexpectedCharacter);
    }

    stream.advance_to(end);
    Ok(Some(Token::Word(&stream.input()[start..end])))
}

fn resolve_string<'a>(
    stream: &mut TokenStream<'a>,
    start: usize,
    remaining: &'a str,
) -> Result<Option<Token<'a>>, TokenizeError> {
    let content_start = start + '"'.len_utf8();

    for (offset, character) in remaining['"'.len_utf8()..].char_indices() {
        if character == '"' {
            let end = content_start + offset;
            let next_position = end + '"'.len_utf8();
            stream.advance_to(next_position);
            return Ok(Some(Token::String(&stream.input()[content_start..end])));
        }
    }

    Err(TokenizeError::UnterminatedString)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_resolver_recognizes_scope_fs_as_word() {
        let mut stream = TokenStream::new("scope-fs \"/tmp\"");

        let token = resolve(&mut stream).unwrap();

        assert_eq!(token, Some(Token::Word("scope-fs")));
    }

    #[test]
    fn token_resolver_recognizes_quoted_path_as_borrowed_string() {
        let input = "scope-fs \"/home/user/documents\"";
        let mut stream = TokenStream::new(input);
        resolve(&mut stream).unwrap();

        let token = resolve(&mut stream).unwrap();

        assert_eq!(token, Some(Token::String("/home/user/documents")));
        let Token::String(path) = token.unwrap() else {
            panic!("expected string token");
        };
        let expected = &input[10..30];
        assert!(std::ptr::eq(path.as_ptr(), expected.as_ptr()));
    }

    #[test]
    fn tokenization_returns_none_at_end() {
        let mut stream = TokenStream::new("scope-fs");

        assert!(resolve(&mut stream).unwrap().is_some());
        assert_eq!(resolve(&mut stream).unwrap(), None);
    }

    #[test]
    fn token_resolver_recognizes_pipeline_separator() {
        let mut stream = TokenStream::new("iter |> take");
        resolve(&mut stream).unwrap();

        let token = resolve(&mut stream).unwrap();

        assert_eq!(token, Some(Token::PipelineSeparator));
    }

    #[test]
    fn token_resolver_recognizes_comma_as_separate_token() {
        let mut stream = TokenStream::new("select name, size");
        resolve(&mut stream).unwrap();
        resolve(&mut stream).unwrap();

        let token = resolve(&mut stream).unwrap();

        assert_eq!(token, Some(Token::Comma));
    }

    #[test]
    fn token_resolver_recognizes_parentheses_as_separate_tokens() {
        let mut stream = TokenStream::new("filter (name equals \"x\")");
        resolve(&mut stream).unwrap();

        let left = resolve(&mut stream).unwrap();
        resolve(&mut stream).unwrap();
        resolve(&mut stream).unwrap();
        resolve(&mut stream).unwrap();
        let right = resolve(&mut stream).unwrap();

        assert_eq!(left, Some(Token::LeftParen));
        assert_eq!(right, Some(Token::RightParen));
    }

    #[test]
    fn unterminated_quote_returns_tokenize_error() {
        let mut stream = TokenStream::new("scope-fs \"/tmp");
        resolve(&mut stream).unwrap();

        let result = resolve(&mut stream);

        assert!(matches!(result, Err(TokenizeError::UnterminatedString)));
    }
}
