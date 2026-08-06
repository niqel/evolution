use evo_shell_engine::SelectProperty;

use crate::definitions::domain::entities::token::Token;
use crate::definitions::domain::entities::token_stream::TokenStream;
use crate::definitions::domain::value_objects::pipeline::{Pipeline, PipelineOperation};
use crate::definitions::use_cases::parse::ParseError;
use crate::definitions::use_cases::tokenize::Tokenize;

pub fn resolve<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
    first_stage: &'a str,
) -> Result<Pipeline, ParseError<'a>> {
    let mut operations = vec![resolve_stage(first_stage, stream, tokenize)?];

    loop {
        let next_token = peek(stream, tokenize)?;
        let Some(next_token) = next_token else {
            break;
        };

        match next_token {
            Token::PipelineSeparator => {
                let _ = tokenize(stream).map_err(ParseError::Tokenize)?;
                let stage_token = tokenize(stream).map_err(ParseError::Tokenize)?;
                let Some(stage_token) = stage_token else {
                    return Err(ParseError::UnexpectedPipelineSeparator);
                };

                let Token::Word(stage_name) = stage_token else {
                    return Err(ParseError::EmptyPipelineStage);
                };

                operations.push(resolve_stage(stage_name, stream, tokenize)?);
            }
            Token::Comma => return Err(ParseError::UnexpectedPipelineArgument(first_stage)),
            _ => return Err(ParseError::UnexpectedPipelineArgument(first_stage)),
        }
    }

    Ok(Pipeline::new(operations))
}

fn resolve_stage<'a>(
    stage_name: &'a str,
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<PipelineOperation, ParseError<'a>> {
    match stage_name {
        "iter" => resolve_iter(stream, tokenize),
        "index" => resolve_index(stream, tokenize),
        "take" => resolve_take(stream, tokenize),
        "select" => resolve_select(stream, tokenize),
        "to-value" => resolve_to_value(stream, tokenize),
        "to-values" => resolve_to_values(stream, tokenize),
        "to-args" => resolve_to_args(stream, tokenize),
        "filter" => Err(ParseError::UnknownPipelineOperation(stage_name)),
        _ => Err(ParseError::UnknownPipelineOperation(stage_name)),
    }
}

fn resolve_iter<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<PipelineOperation, ParseError<'a>> {
    ensure_no_stage_arguments(stream, tokenize, "iter")?;
    Ok(PipelineOperation::Iter)
}

fn resolve_index<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<PipelineOperation, ParseError<'a>> {
    let value = parse_single_usize(stream, tokenize, "index")?;
    Ok(PipelineOperation::Index(value))
}

fn resolve_take<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<PipelineOperation, ParseError<'a>> {
    let value = parse_single_usize(stream, tokenize, "take")?;
    Ok(PipelineOperation::Take(value))
}

fn resolve_select<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<PipelineOperation, ParseError<'a>> {
    let mut properties = Vec::new();
    properties.push(parse_select_property(next_word_token(
        stream, tokenize, "select",
    )?)?);

    loop {
        let next_token = peek(stream, tokenize)?;

        match next_token {
            None | Some(Token::PipelineSeparator) => break,
            Some(Token::Comma) => {
                let _ = tokenize(stream).map_err(ParseError::Tokenize)?;
                properties.push(parse_select_property(next_word_token(
                    stream, tokenize, "select",
                )?)?);
            }
            Some(_) => return Err(ParseError::UnexpectedPipelineArgument("select")),
        }
    }

    Ok(PipelineOperation::Select(properties))
}

fn resolve_to_value<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<PipelineOperation, ParseError<'a>> {
    ensure_no_stage_arguments(stream, tokenize, "to-value")?;
    Ok(PipelineOperation::ToValue)
}

fn resolve_to_values<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<PipelineOperation, ParseError<'a>> {
    ensure_no_stage_arguments(stream, tokenize, "to-values")?;
    Ok(PipelineOperation::ToValues)
}

fn resolve_to_args<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<PipelineOperation, ParseError<'a>> {
    ensure_no_stage_arguments(stream, tokenize, "to-args")?;
    Ok(PipelineOperation::ToArgs)
}

fn parse_single_usize<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
    stage_name: &'a str,
) -> Result<usize, ParseError<'a>> {
    let value = next_word_token(stream, tokenize, stage_name)?;
    let parsed = value
        .parse::<usize>()
        .map_err(|_| ParseError::InvalidPipelineArgument(stage_name))?;
    ensure_no_stage_arguments(stream, tokenize, stage_name)?;
    Ok(parsed)
}

fn parse_select_property<'a>(property: &'a str) -> Result<SelectProperty, ParseError<'a>> {
    match property {
        "index" => Ok(SelectProperty::Index),
        "created" => Ok(SelectProperty::Created),
        "modified" => Ok(SelectProperty::Modified),
        "type" => Ok(SelectProperty::Type),
        "size" => Ok(SelectProperty::Size),
        "name" => Ok(SelectProperty::Name),
        _ => Err(ParseError::UnsupportedSelectProperty(property)),
    }
}

fn next_word_token<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
    stage_name: &'a str,
) -> Result<&'a str, ParseError<'a>> {
    let token = tokenize(stream).map_err(ParseError::Tokenize)?;

    let Some(token) = token else {
        return Err(ParseError::MissingPipelineArgument(stage_name));
    };

    match token {
        Token::Word(word) => Ok(word),
        Token::String(word) => Err(ParseError::InvalidPipelineArgument(word)),
        Token::PipelineSeparator => Err(ParseError::MissingPipelineArgument(stage_name)),
        Token::Comma => Err(ParseError::MissingPipelineArgument(stage_name)),
    }
}

fn ensure_no_stage_arguments<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
    stage_name: &'a str,
) -> Result<(), ParseError<'a>> {
    let next_token = peek(stream, tokenize)?;

    match next_token {
        None | Some(Token::PipelineSeparator) => Ok(()),
        Some(_) => Err(ParseError::UnexpectedPipelineArgument(stage_name)),
    }
}

fn peek<'a>(
    stream: &mut TokenStream<'a>,
    tokenize: Tokenize,
) -> Result<Option<Token<'a>>, ParseError<'a>> {
    let position = stream.position();
    let token = tokenize(stream).map_err(ParseError::Tokenize)?;
    stream.advance_to(position);
    Ok(token)
}
