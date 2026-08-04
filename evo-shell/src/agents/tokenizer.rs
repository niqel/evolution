use crate::definitions::domain::entities::token::Token;
use crate::definitions::domain::entities::token_stream::TokenStream;
use crate::definitions::use_cases::tokenize::TokenizeError;
use crate::resolvers::token;

pub fn tokenize<'a>(stream: &mut TokenStream<'a>) -> Result<Option<Token<'a>>, TokenizeError> {
    token::resolve(stream)
}
