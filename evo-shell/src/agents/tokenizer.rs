use crate::definitions::domain::entities::token::Token;
use crate::definitions::domain::entities::token_stream::TokenStream;
use crate::definitions::use_cases::tokenize::TokenizeError;
use crate::resolvers::token;

pub fn tokenize<'a>(stream: &mut TokenStream<'a>) -> Result<Option<Token<'a>>, TokenizeError> {
    token::resolve(stream)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::use_cases::tokenize::Tokenize;

    #[test]
    fn tokenizer_matches_tokenize_function_pointer() {
        let tokenize: Tokenize = tokenize;
        let mut stream = TokenStream::new("scope-fs");

        let token = tokenize(&mut stream).unwrap();

        assert_eq!(token, Some(Token::Word("scope-fs")));
    }
}
