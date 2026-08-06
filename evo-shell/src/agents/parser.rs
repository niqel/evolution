use crate::definitions::domain::entities::command::Command;
use crate::definitions::domain::entities::token_stream::TokenStream;
use crate::definitions::use_cases::parse::ParseError;
use crate::definitions::use_cases::tokenize::Tokenize;
use crate::resolvers::command;

pub fn parse<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<Command<'a>, ParseError<'a>> {
    let command = command::resolve(stream, tokenize)?;

    if tokenize(stream).map_err(ParseError::Tokenize)?.is_some() {
        return Err(ParseError::UnexpectedToken);
    }

    Ok(command)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::tokenizer;
    use crate::definitions::use_cases::parse::Parse;

    #[test]
    fn parser_matches_parse_function_pointer() {
        let parse: Parse = parse;
        let mut stream = TokenStream::new("scope-fs \"/ruta\"");

        let command = parse(&mut stream, tokenizer::tokenize).unwrap();

        assert_eq!(command, Command::ScopeFs("/ruta"));
    }
}
