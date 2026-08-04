use crate::definitions::domain::entities::command::Command;
use crate::definitions::domain::entities::token::Token;
use crate::definitions::domain::entities::token_stream::TokenStream;
use crate::definitions::use_cases::parse::ParseError;
use crate::definitions::use_cases::tokenize::Tokenize;

pub fn resolve<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<Command<'a>, ParseError<'a>> {
    let command_token = tokenize(stream).map_err(ParseError::Tokenize)?;

    let Some(command_token) = command_token else {
        return Err(ParseError::ExpectedCommand);
    };

    let Token::Word(command_name) = command_token else {
        return Err(ParseError::InvalidCommandToken(command_token));
    };

    if command_name != "scope-fs" {
        return Err(ParseError::UnknownCommand(command_name));
    }

    let path_token = tokenize(stream).map_err(ParseError::Tokenize)?;

    let Some(path_token) = path_token else {
        return Err(ParseError::ExpectedPath);
    };

    let Token::String(path) = path_token else {
        return Err(ParseError::UnexpectedToken);
    };

    if tokenize(stream).map_err(ParseError::Tokenize)?.is_some() {
        return Err(ParseError::UnexpectedToken);
    }

    Ok(Command::ScopeFs(path))
}
