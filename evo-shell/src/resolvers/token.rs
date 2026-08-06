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
