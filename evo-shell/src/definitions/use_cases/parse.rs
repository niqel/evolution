use crate::definitions::domain::entities::command::Command;
use crate::definitions::domain::entities::token::Token;
use crate::definitions::domain::entities::token_stream::TokenStream;
use crate::definitions::use_cases::tokenize::{Tokenize, TokenizeError};

pub type Parse = for<'a> fn(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<Command<'a>, ParseError<'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError<'a> {
    Tokenize(TokenizeError),
    ExpectedCommand,
    ExpectedPath,
    UnexpectedToken,
    UnknownCommand(&'a str),
    InvalidCommandToken(Token<'a>),
    UnexpectedPipelineSeparator,
    EmptyPipelineStage,
    UnknownPipelineOperation(&'a str),
    MissingPipelineArgument(&'a str),
    InvalidPipelineArgument(&'a str),
    UnsupportedSelectProperty(&'a str),
    UnexpectedPipelineArgument(&'a str),
    EmptyFilterExpression,
    UnknownFilterProperty(&'a str),
    MissingFilterOperator(&'a str),
    UnknownFilterOperator(&'a str),
    MissingFilterValue(&'a str),
    InvalidFilterValue(&'a str),
    UnsupportedFilterValue(&'a str),
    MissingBetweenUpperBound(&'a str),
    UnexpectedFilterToken(Token<'a>),
    UnexpectedClosingParenthesis,
    UnclosedParenthesis,
    AmbiguousLogicalExpression,
}
