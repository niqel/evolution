use crate::definitions::domain::entities::token::Token;
use crate::definitions::domain::entities::token_stream::TokenStream;

pub type Tokenize =
    for<'a> fn(stream: &mut TokenStream<'a>) -> Result<Option<Token<'a>>, TokenizeError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenizeError {
    UnexpectedCharacter,
    UnterminatedString,
}
