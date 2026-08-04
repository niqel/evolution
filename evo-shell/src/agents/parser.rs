use crate::definitions::domain::entities::command::Command;
use crate::definitions::domain::entities::token_stream::TokenStream;
use crate::definitions::use_cases::parse::ParseError;
use crate::definitions::use_cases::tokenize::Tokenize;
use crate::resolvers::command;

pub fn parse<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<Command<'a>, ParseError<'a>> {
    command::resolve(stream, tokenize)
}
