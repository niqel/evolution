use crate::definitions::domain::entities::command::Command;
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

    let Token::Word(command_name) = command_token else {
        return match command_token {
            Token::PipelineSeparator => Err(ParseError::UnexpectedPipelineSeparator),
            Token::Comma => Err(ParseError::EmptyPipelineStage),
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

    if tokenize(stream).map_err(ParseError::Tokenize)?.is_some() {
        return Err(ParseError::UnexpectedToken);
    }

    Ok(Command::ScopeFs(path))
}

fn resolve_iter<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<Command<'a>, ParseError<'a>> {
    if tokenize(stream).map_err(ParseError::Tokenize)?.is_some() {
        return Err(ParseError::UnexpectedToken);
    }

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

    let location = match location_token {
        Token::Word(location) | Token::String(location) => location,
        _ => return Err(ParseError::UnexpectedToken),
    };

    if tokenize(stream).map_err(ParseError::Tokenize)?.is_some() {
        return Err(ParseError::UnexpectedToken);
    }

    Ok(Command::Enter(location))
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

    if tokenize(stream).map_err(ParseError::Tokenize)?.is_some() {
        return Err(ParseError::UnexpectedToken);
    }

    Ok(Command::Clear(TerminalClearMode::All))
}

fn resolve_exit<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<Command<'a>, ParseError<'a>> {
    if tokenize(stream).map_err(ParseError::Tokenize)?.is_some() {
        return Err(ParseError::UnexpectedToken);
    }

    Ok(Command::Exit)
}
