use crate::definitions::domain::entities::command::{Command, CommandArgument};
use crate::definitions::domain::entities::token::Token;
use crate::definitions::domain::entities::token_stream::TokenStream;
use crate::definitions::domain::value_objects::terminal_clear_mode::TerminalClearMode;
use crate::definitions::use_cases::parse::ParseError;
use crate::definitions::use_cases::tokenize::Tokenize;
use crate::resolvers::pipeline;

pub fn resolve<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<Command<'a>, ParseError<'a>> {
    let command_token = tokenize(stream).map_err(ParseError::Tokenize)?;

    let Some(command_token) = command_token else {
        return Err(ParseError::ExpectedCommand);
    };

    if matches!(command_token, Token::LeftParen) {
        return resolve_grouped(stream, tokenize);
    }

    let Token::Word(command_name) = command_token else {
        return match command_token {
            Token::PipelineSeparator => Err(ParseError::UnexpectedPipelineSeparator),
            Token::Comma => Err(ParseError::EmptyPipelineStage),
            Token::RightParen => Err(ParseError::UnexpectedClosingParenthesis),
            _ => Err(ParseError::InvalidCommandToken(command_token)),
        };
    };

    let position = stream.position();
    let next_token = tokenize(stream).map_err(ParseError::Tokenize)?;
    let has_pipeline_separator = matches!(next_token, Some(Token::PipelineSeparator));
    stream.advance_to(position);

    if has_pipeline_separator {
        return resolve_pipeline(stream, tokenize, command_name);
    }

    match command_name {
        "scope-fs" => resolve_scope_fs(stream, tokenize),
        "iter" => resolve_iter(stream, tokenize),
        "enter" => resolve_enter(stream, tokenize),
        "copy-to" => resolve_copy_to(stream, tokenize),
        "clear" => resolve_clear(stream, tokenize),
        "exit" => resolve_exit(stream, tokenize),
        "take" | "index" | "select" | "to-value" | "to-values" | "to-args" | "filter" => {
            resolve_pipeline(stream, tokenize, command_name)
        }
        _ => Err(ParseError::UnknownCommand(command_name)),
    }
}

fn resolve_pipeline<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
    stage_name: &'a str,
) -> Result<Command<'a>, ParseError<'a>> {
    pipeline::resolve(stream, tokenize, stage_name).map(Command::Pipeline)
}

fn resolve_scope_fs<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<Command<'a>, ParseError<'a>> {
    let path_token = tokenize(stream).map_err(ParseError::Tokenize)?;

    let Some(path_token) = path_token else {
        return Err(ParseError::ExpectedPath);
    };

    let Token::String(path) = path_token else {
        return Err(ParseError::UnexpectedToken);
    };

    Ok(Command::ScopeFs(path))
}

fn resolve_iter<'a>(
    _stream: &mut TokenStream<'a>,
    _tokenize: Tokenize,
) -> Result<Command<'a>, ParseError<'a>> {
    Ok(Command::Iter)
}

fn resolve_enter<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<Command<'a>, ParseError<'a>> {
    let location_token = tokenize(stream).map_err(ParseError::Tokenize)?;

    let Some(location_token) = location_token else {
        return Err(ParseError::ExpectedPath);
    };

    let argument = match location_token {
        Token::LeftParen => {
            let inner = resolve(stream, tokenize)?;
            let closing = tokenize(stream).map_err(ParseError::Tokenize)?;
            let Some(Token::RightParen) = closing else {
                return Err(ParseError::UnclosedParenthesis);
            };
            CommandArgument::Grouped(Box::new(inner))
        }
        Token::Word(location) | Token::String(location) => CommandArgument::Literal(location),
        _ => return Err(ParseError::UnexpectedToken),
    };

    Ok(Command::Enter(argument))
}

fn resolve_clear<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<Command<'a>, ParseError<'a>> {
    let option_token = tokenize(stream).map_err(ParseError::Tokenize)?;

    let Some(option_token) = option_token else {
        return Ok(Command::Clear(TerminalClearMode::Visible));
    };

    let Token::Word("--all") = option_token else {
        return Err(ParseError::UnexpectedToken);
    };

    Ok(Command::Clear(TerminalClearMode::All))
}

fn resolve_exit<'a>(
    _stream: &mut TokenStream<'a>,
    _tokenize: Tokenize,
) -> Result<Command<'a>, ParseError<'a>> {
    Ok(Command::Exit)
}

fn resolve_grouped<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<Command<'a>, ParseError<'a>> {
    let inner = resolve(stream, tokenize)?;

    let closing_token = tokenize(stream).map_err(ParseError::Tokenize)?;
    let Some(Token::RightParen) = closing_token else {
        return Err(ParseError::UnclosedParenthesis);
    };

    Ok(Command::Grouped(Box::new(inner)))
}

fn resolve_copy_to<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<Command<'a>, ParseError<'a>> {
    let mut sources = Vec::new();
    let mut destination = None;

    loop {
        let token = tokenize(stream).map_err(ParseError::Tokenize)?;

        let Some(token) = token else {
            break;
        };

        if let Token::Word("path") = token {
            let next_pos = stream.position();
            let next_token = tokenize(stream).map_err(ParseError::Tokenize)?;
            if matches!(next_token, Some(Token::Colon)) {
                let dest_token = tokenize(stream).map_err(ParseError::Tokenize)?;
                let Some(dest_token) = dest_token else {
                    return Err(ParseError::ExpectedPath);
                };

                destination = Some(parse_command_argument(dest_token, stream, tokenize)?);
                break;
            }
            stream.advance_to(next_pos);
        } else if let Token::Word(other_named) = token {
            let next_pos = stream.position();
            let next_token = tokenize(stream).map_err(ParseError::Tokenize)?;
            if matches!(next_token, Some(Token::Colon)) {
                return Err(ParseError::UnknownNamedArgument(other_named));
            }
            stream.advance_to(next_pos);
        }

        let arg = parse_command_argument(token, stream, tokenize)?;
        sources.push(arg);

        let comma_pos = stream.position();
        let comma_token = tokenize(stream).map_err(ParseError::Tokenize)?;
        if matches!(comma_token, Some(Token::Comma)) {
            continue;
        }
        stream.advance_to(comma_pos);
    }

    let Some(destination) = destination else {
        return Err(ParseError::ExpectedPath);
    };

    if sources.is_empty() {
        return Err(ParseError::MissingSource);
    }

    Ok(Command::CopyTo {
        sources,
        destination,
    })
}

fn parse_command_argument<'a>(
    token: Token<'a>,
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<CommandArgument<'a>, ParseError<'a>> {
    match token {
        Token::LeftParen => {
            let inner = resolve(stream, tokenize)?;
            let closing = tokenize(stream).map_err(ParseError::Tokenize)?;
            let Some(Token::RightParen) = closing else {
                return Err(ParseError::UnclosedParenthesis);
            };
            Ok(CommandArgument::Grouped(Box::new(inner)))
        }
        Token::Word(location) | Token::String(location) => Ok(CommandArgument::Literal(location)),
        _ => Err(ParseError::UnexpectedToken),
    }
}
