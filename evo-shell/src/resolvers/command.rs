use crate::definitions::domain::entities::command::{Command, CommandArgument};
use crate::definitions::domain::entities::token::Token;
use crate::definitions::domain::entities::token_stream::TokenStream;
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
    let next_token = tokenize(stream).map_err(ParseError::Tokenize)?;

    if next_token.is_some() {
        return Err(ParseError::UnexpectedToken);
    }

    Ok(Command::Clear)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agents::{parser, tokenizer};
    use crate::definitions::domain::value_objects::pipeline::PipelineOperation;
    use evo_shell_engine::{
        FilterComparison, FilterExpression, FilterOperand, FilterOperator, FilterProperty,
        FilterValue, SelectProperty,
    };

    #[test]
    fn parser_resolves_scope_fs_command() {
        let mut stream = TokenStream::new("scope-fs \"/ruta\"");

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        assert_eq!(command, Command::ScopeFs("/ruta"));
    }

    #[test]
    fn parser_resolves_iter_command() {
        let mut stream = TokenStream::new("iter");

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        assert_eq!(command, Command::Iter);
    }

    #[test]
    fn parser_accepts_clear_without_arguments() {
        let mut stream = TokenStream::new("clear");

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        assert_eq!(command, Command::Clear);
    }

    #[test]
    fn parser_rejects_clear_all() {
        let mut stream = TokenStream::new("clear --all");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert_eq!(result, Err(ParseError::UnexpectedToken));
    }

    #[test]
    fn parser_rejects_clear_extra_argument() {
        let mut stream = TokenStream::new("clear foo");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert_eq!(result, Err(ParseError::UnexpectedToken));
    }

    #[test]
    fn parser_resolves_exit_command() {
        let mut stream = TokenStream::new("exit");

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        assert_eq!(command, Command::Exit);
    }

    #[test]
    fn parser_resolves_basic_pipeline_to_value() {
        let mut stream = TokenStream::new("iter |> take 1 |> select name |> to-value");

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let Command::Pipeline(pipeline) = command else {
            panic!("expected pipeline command");
        };

        assert_eq!(
            pipeline.operations(),
            &[
                PipelineOperation::Iter,
                PipelineOperation::Take(1),
                PipelineOperation::Select(vec![SelectProperty::Name]),
                PipelineOperation::ToValue,
            ]
        );
    }

    #[test]
    fn parser_resolves_filter_pipeline_equals() {
        let mut stream = TokenStream::new(
            r#"iter |> filter name equals "file.txt" |> select name |> to-values"#,
        );

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let Command::Pipeline(pipeline) = command else {
            panic!("expected pipeline command");
        };

        assert_eq!(
            pipeline.operations(),
            &[
                PipelineOperation::Iter,
                PipelineOperation::Filter(FilterExpression::comparison(FilterComparison::new(
                    FilterProperty::Name,
                    FilterOperator::Equals,
                    FilterOperand::single(FilterValue::name("file.txt")),
                ))),
                PipelineOperation::Select(vec![SelectProperty::Name]),
                PipelineOperation::ToValues,
            ]
        );
    }

    #[test]
    fn parser_resolves_filter_pipeline_not_equals() {
        let mut stream = TokenStream::new(
            r#"iter |> filter name not-equals "alpha.txt" |> select name |> to-values"#,
        );

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let Command::Pipeline(pipeline) = command else {
            panic!("expected pipeline command");
        };

        assert_eq!(
            pipeline.operations(),
            &[
                PipelineOperation::Iter,
                PipelineOperation::Filter(FilterExpression::comparison(FilterComparison::new(
                    FilterProperty::Name,
                    FilterOperator::NotEquals,
                    FilterOperand::single(FilterValue::name("alpha.txt")),
                ))),
                PipelineOperation::Select(vec![SelectProperty::Name]),
                PipelineOperation::ToValues,
            ]
        );
    }

    #[test]
    fn parser_resolves_filter_pipeline_continuation() {
        let mut stream =
            TokenStream::new(r#"iter |> filter size > 10kb |> take 1 |> select name |> to-value"#);

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let Command::Pipeline(pipeline) = command else {
            panic!("expected pipeline command");
        };

        assert_eq!(
            pipeline.operations(),
            &[
                PipelineOperation::Iter,
                PipelineOperation::Filter(FilterExpression::comparison(FilterComparison::new(
                    FilterProperty::Size,
                    FilterOperator::GreaterThan,
                    FilterOperand::single(FilterValue::size(10_000)),
                ))),
                PipelineOperation::Take(1),
                PipelineOperation::Select(vec![SelectProperty::Name]),
                PipelineOperation::ToValue,
            ]
        );
    }

    #[test]
    fn parser_resolves_pipeline_with_multiple_select_properties() {
        let mut stream = TokenStream::new("iter |> select name, size |> to-values");

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let Command::Pipeline(pipeline) = command else {
            panic!("expected pipeline command");
        };

        assert_eq!(
            pipeline.operations(),
            &[
                PipelineOperation::Iter,
                PipelineOperation::Select(vec![SelectProperty::Name, SelectProperty::Size]),
                PipelineOperation::ToValues,
            ]
        );
    }

    #[test]
    fn parser_resolves_pipeline_with_compact_select_comma() {
        let mut stream = TokenStream::new("iter |> select name,size |> to-values");

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let Command::Pipeline(pipeline) = command else {
            panic!("expected pipeline command");
        };

        assert_eq!(
            pipeline.operations(),
            &[
                PipelineOperation::Iter,
                PipelineOperation::Select(vec![SelectProperty::Name, SelectProperty::Size]),
                PipelineOperation::ToValues,
            ]
        );
    }

    #[test]
    fn parser_preserves_duplicate_select_properties() {
        let mut stream = TokenStream::new("iter |> select name, name |> to-values");

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let Command::Pipeline(pipeline) = command else {
            panic!("expected pipeline command");
        };

        assert_eq!(
            pipeline.operations(),
            &[
                PipelineOperation::Iter,
                PipelineOperation::Select(vec![SelectProperty::Name, SelectProperty::Name]),
                PipelineOperation::ToValues,
            ]
        );
    }

    #[test]
    fn parser_resolves_pipeline_to_args() {
        let mut stream = TokenStream::new("iter |> select name |> to-args");

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let Command::Pipeline(pipeline) = command else {
            panic!("expected pipeline command");
        };

        assert_eq!(
            pipeline.operations(),
            &[
                PipelineOperation::Iter,
                PipelineOperation::Select(vec![SelectProperty::Name]),
                PipelineOperation::ToArgs,
            ]
        );
    }

    #[test]
    fn parser_accepts_pipeline_without_spaces_around_separator() {
        let mut stream = TokenStream::new("iter|>take 1|>select name|>to-value");

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let Command::Pipeline(pipeline) = command else {
            panic!("expected pipeline command");
        };

        assert_eq!(
            pipeline.operations(),
            &[
                PipelineOperation::Iter,
                PipelineOperation::Take(1),
                PipelineOperation::Select(vec![SelectProperty::Name]),
                PipelineOperation::ToValue,
            ]
        );
    }

    #[test]
    fn parser_resolves_pipeline_with_index_stage() {
        let mut stream = TokenStream::new("iter |> index 0 |> select name |> to-value");

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let Command::Pipeline(pipeline) = command else {
            panic!("expected pipeline command");
        };

        assert_eq!(
            pipeline.operations(),
            &[
                PipelineOperation::Iter,
                PipelineOperation::Index(0),
                PipelineOperation::Select(vec![SelectProperty::Name]),
                PipelineOperation::ToValue,
            ]
        );
    }

    #[test]
    fn parser_resolves_pipeline_with_take_zero() {
        let mut stream = TokenStream::new("iter |> take 0 |> select name |> to-values");

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let Command::Pipeline(pipeline) = command else {
            panic!("expected pipeline command");
        };

        assert_eq!(
            pipeline.operations(),
            &[
                PipelineOperation::Iter,
                PipelineOperation::Take(0),
                PipelineOperation::Select(vec![SelectProperty::Name]),
                PipelineOperation::ToValues,
            ]
        );
    }

    #[test]
    fn parser_accepts_syntactically_valid_but_semantically_invalid_pipeline() {
        let mut stream = TokenStream::new("iter |> to-value");

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let Command::Pipeline(pipeline) = command else {
            panic!("expected pipeline command");
        };

        assert_eq!(
            pipeline.operations(),
            &[PipelineOperation::Iter, PipelineOperation::ToValue]
        );
    }

    #[test]
    fn parser_rejects_exit_extra_token() {
        let mut stream = TokenStream::new("exit now");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(result, Err(ParseError::UnexpectedToken)));
    }

    #[test]
    fn parser_rejects_exit_option() {
        let mut stream = TokenStream::new("exit --force");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(result, Err(ParseError::UnexpectedToken)));
    }

    #[test]
    fn parser_rejects_exit_numeric_argument() {
        let mut stream = TokenStream::new("exit 0");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(result, Err(ParseError::UnexpectedToken)));
    }

    #[test]
    fn parser_rejects_trailing_pipeline_separator() {
        let mut stream = TokenStream::new("iter |>");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(
            result,
            Err(ParseError::UnexpectedPipelineSeparator)
        ));
    }

    #[test]
    fn parser_rejects_empty_pipeline_stage() {
        let mut stream = TokenStream::new("iter |> |> take 1");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(result, Err(ParseError::EmptyPipelineStage)));
    }

    #[test]
    fn parser_rejects_leading_pipeline_separator() {
        let mut stream = TokenStream::new("|> iter");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(
            result,
            Err(ParseError::UnexpectedPipelineSeparator)
        ));
    }

    #[test]
    fn parser_rejects_unknown_pipeline_operation() {
        let mut stream = TokenStream::new("iter |> foo");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(
            result,
            Err(ParseError::UnknownPipelineOperation("foo"))
        ));
    }

    #[test]
    fn parser_rejects_take_without_argument() {
        let mut stream = TokenStream::new("iter |> take");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(
            result,
            Err(ParseError::MissingPipelineArgument("take"))
        ));
    }

    #[test]
    fn parser_rejects_take_with_invalid_argument() {
        let mut stream = TokenStream::new("iter |> take hello");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(
            result,
            Err(ParseError::InvalidPipelineArgument("take"))
        ));
    }

    #[test]
    fn parser_rejects_take_with_negative_argument() {
        let mut stream = TokenStream::new("iter |> take -1");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(
            result,
            Err(ParseError::InvalidPipelineArgument("take"))
        ));
    }

    #[test]
    fn parser_rejects_index_without_argument() {
        let mut stream = TokenStream::new("iter |> index");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(
            result,
            Err(ParseError::MissingPipelineArgument("index"))
        ));
    }

    #[test]
    fn parser_rejects_index_with_invalid_argument() {
        let mut stream = TokenStream::new("iter |> index hello");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(
            result,
            Err(ParseError::InvalidPipelineArgument("index"))
        ));
    }

    #[test]
    fn parser_rejects_select_without_properties() {
        let mut stream = TokenStream::new("iter |> select");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(
            result,
            Err(ParseError::MissingPipelineArgument("select"))
        ));
    }

    #[test]
    fn parser_rejects_invalid_select_property() {
        let mut stream = TokenStream::new("iter |> select foo");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(
            result,
            Err(ParseError::UnsupportedSelectProperty("foo"))
        ));
    }

    #[test]
    fn parser_rejects_to_value_with_argument() {
        let mut stream = TokenStream::new("iter |> to-value extra");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(
            result,
            Err(ParseError::UnexpectedPipelineArgument("to-value"))
        ));
    }

    #[test]
    fn parser_resolves_enter_word_location() {
        let mut stream = TokenStream::new("enter agents");

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        assert_eq!(command, Command::Enter(CommandArgument::Literal("agents")));
    }

    #[test]
    fn parser_resolves_enter_quoted_location() {
        let mut stream = TokenStream::new("enter \"Mis Documentos\"");

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        assert_eq!(
            command,
            Command::Enter(CommandArgument::Literal("Mis Documentos"))
        );
    }

    #[test]
    fn parser_resolves_enter_parent_location() {
        let mut stream = TokenStream::new("enter ..");

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        assert_eq!(command, Command::Enter(CommandArgument::Literal("..")));
    }

    #[test]
    fn parser_resolves_enter_two_parents_location() {
        let mut stream = TokenStream::new("enter ../..");

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        assert_eq!(command, Command::Enter(CommandArgument::Literal("../..")));
    }

    #[test]
    fn parser_rejects_missing_path() {
        let mut stream = TokenStream::new("scope-fs");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(result, Err(ParseError::ExpectedPath)));
    }

    #[test]
    fn parser_rejects_extra_token() {
        let mut stream = TokenStream::new("scope-fs \"/ruta\" extra");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(result, Err(ParseError::UnexpectedToken)));
    }

    #[test]
    fn parser_rejects_unknown_command() {
        let mut stream = TokenStream::new("unknown-command \"/ruta\"");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(
            result,
            Err(ParseError::UnknownCommand("unknown-command"))
        ));
    }

    #[test]
    fn parser_rejects_iter_extra_token() {
        let mut stream = TokenStream::new("iter extra");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(result, Err(ParseError::UnexpectedToken)));
    }

    #[test]
    fn parser_rejects_iter_quoted_argument() {
        let mut stream = TokenStream::new("iter \"/ruta\"");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(result, Err(ParseError::UnexpectedToken)));
    }

    #[test]
    fn parser_rejects_clear_positional_argument() {
        let mut stream = TokenStream::new("clear all");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(result, Err(ParseError::UnexpectedToken)));
    }

    #[test]
    fn parser_rejects_clear_unknown_option() {
        let mut stream = TokenStream::new("clear --unknown");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(result, Err(ParseError::UnexpectedToken)));
    }

    #[test]
    fn parser_rejects_clear_all_extra_token() {
        let mut stream = TokenStream::new("clear --all extra");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(result, Err(ParseError::UnexpectedToken)));
    }

    #[test]
    fn parser_rejects_enter_without_location() {
        let mut stream = TokenStream::new("enter");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(result, Err(ParseError::ExpectedPath)));
    }

    #[test]
    fn parser_rejects_enter_extra_word_argument() {
        let mut stream = TokenStream::new("enter agents extra");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(result, Err(ParseError::UnexpectedToken)));
    }

    #[test]
    fn parser_rejects_enter_extra_after_quoted_location() {
        let mut stream = TokenStream::new("enter \"Mis Documentos\" extra");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(result, Err(ParseError::UnexpectedToken)));
    }

    #[test]
    fn parser_rejects_empty_input() {
        let mut stream = TokenStream::new("");

        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert!(matches!(result, Err(ParseError::ExpectedCommand)));
    }

    #[test]
    fn parser_consumes_tokens_incrementally_without_token_vec() {
        let mut stream = TokenStream::new("scope-fs \"/ruta\"");

        parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        assert_eq!(stream.position(), stream.input().len());
    }
}
