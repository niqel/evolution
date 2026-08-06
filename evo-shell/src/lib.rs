mod agents;
mod definitions;
pub mod presentation_style;
mod providers;
mod resolvers;

pub use agents::{
    executor, exiter, iteration_presenter, parser, pipeline_executor, pipeline_result_presenter,
    shell_initializer, starter, terminal_clearer, tokenizer, welcome_presenter,
};
pub use definitions::domain::entities::command::{Command, CommandArgument};
pub use definitions::domain::entities::shell::Shell;
pub use definitions::domain::entities::token::Token;
pub use definitions::domain::entities::token_stream::TokenStream;
pub use definitions::domain::value_objects::pipeline::{
    Pipeline, PipelineOperation, PipelineOperationKind,
};
pub use definitions::domain::value_objects::pipeline_value::{
    PipelineItems, PipelineValue, PipelineValueKind,
};
pub use definitions::domain::value_objects::terminal_clear_mode::TerminalClearMode;
pub use definitions::use_cases::execute::{Execute, ExecuteError, ExecutionResult};
pub use definitions::use_cases::execute_pipeline::{ExecutePipeline, PipelineExecutionError};
pub use definitions::use_cases::exiter::Exit;
pub use definitions::use_cases::initialize_shell::{InitializeShell, InitializeShellError};
pub use definitions::use_cases::parse::{Parse, ParseError};
pub use definitions::use_cases::pipeline_result_presenter::{
    PipelineResultPresentError, PresentPipelineResult,
};
pub use definitions::use_cases::starter::{Start, StartError};
pub use definitions::use_cases::terminal_clearer::{TerminalClearError, TerminalClearer};
pub use definitions::use_cases::tokenize::{Tokenize, TokenizeError};
pub use definitions::use_cases::welcome_presenter::{WelcomePresenter, WelcomePresenterError};

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::ffi::OsStr;
    use std::fs;
    use std::io;
    use std::path::Path;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::SystemTime;

    use evo_shell_engine::{
        FilesystemEntryKind, FilterComparison, FilterExpression, FilterOperand, FilterOperator,
        FilterProperty, FilterValue, IterError, ProjectedValue, ScopeError, SelectProperty,
        SetFilesystemScope, iteration_advancer, scope_setter,
    };

    use crate::definitions::contracts::current_directory::{
        CurrentDirectory, CurrentDirectoryError,
    };
    use crate::definitions::domain::entities::shell::Shell;
    use crate::definitions::providers::terminal_clearer::Provide;
    use crate::definitions::resolvers::terminal_clearer::Resolve;
    use crate::definitions::use_cases::pipeline_result_presenter::PipelineResultPresentError;
    use crate::definitions::use_cases::terminal_clearer::TerminalClearError;
    use crate::providers::terminal_clearer as terminal_clearer_provider;
    use crate::resolvers::execution;
    use crate::resolvers::pipeline_result_presenter as pipeline_result_presenter_resolver;
    use crate::resolvers::shell;
    use crate::resolvers::terminal_clearer as terminal_clearer_resolver;
    use crate::resolvers::token;
    use crate::{
        Command, CommandArgument, Execute, ExecuteError, ExecutionResult, Exit, InitializeShell,
        InitializeShellError, Parse, ParseError, Pipeline, PipelineExecutionError,
        PipelineOperation, PipelineOperationKind, PipelineValue, PipelineValueKind,
        TerminalClearMode, TerminalClearer, Token, TokenStream, Tokenize, TokenizeError, executor,
        exiter, parser, pipeline_executor, pipeline_result_presenter, shell_initializer,
        terminal_clearer, tokenizer,
    };

    struct TestDirectory {
        path: PathBuf,
    }

    struct FailingWriter;

    thread_local! {
        static CAPTURED_OUTPUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    }

    impl io::Write for FailingWriter {
        fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
            Err(io::Error::other("write failed"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl TestDirectory {
        fn new(name: &str) -> Self {
            let unique = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("system time should be after UNIX_EPOCH")
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "evo_shell_{name}_{}_{}",
                std::process::id(),
                unique
            ));

            fs::create_dir(&path).expect("temporary test directory should be created");

            Self { path }
        }
    }

    fn current_directory() -> Result<PathBuf, CurrentDirectoryError> {
        std::env::current_dir()
    }

    fn current_directory_error() -> Result<PathBuf, CurrentDirectoryError> {
        Err(io::Error::other("current directory failed"))
    }

    fn scope_error(path: &Path) -> Result<evo_shell_engine::FilesystemScope, ScopeError> {
        Err(ScopeError::NotDirectory(path.to_path_buf()))
    }

    fn shell_from_directory(directory: &TestDirectory) -> Shell {
        Shell::new(scope_setter::set(directory.path.as_path()).unwrap())
    }

    fn capture_rendered(rendered: &str) -> Result<(), PipelineResultPresentError> {
        CAPTURED_OUTPUT.with(|buffer| {
            buffer.borrow_mut().extend_from_slice(rendered.as_bytes());
        });

        Ok(())
    }

    fn captured_rendered_output() -> String {
        CAPTURED_OUTPUT.with(|buffer| {
            String::from_utf8(buffer.borrow().clone()).expect("rendered output should be utf8")
        })
    }

    fn clear_captured_output() {
        CAPTURED_OUTPUT.with(|buffer| {
            buffer.borrow_mut().clear();
        });
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn token_stream_borrows_input_and_starts_at_initial_position() {
        let input = "scope-fs \"/tmp\"";
        let stream = TokenStream::new(input);

        assert_eq!(stream.input(), input);
        assert_eq!(stream.position(), 0);
    }

    #[test]
    fn token_resolver_recognizes_scope_fs_as_word() {
        let mut stream = TokenStream::new("scope-fs \"/tmp\"");

        let token = token::resolve(&mut stream).unwrap();

        assert_eq!(token, Some(Token::Word("scope-fs")));
    }

    #[test]
    fn token_resolver_recognizes_quoted_path_as_borrowed_string() {
        let input = "scope-fs \"/home/user/documents\"";
        let mut stream = TokenStream::new(input);
        token::resolve(&mut stream).unwrap();

        let token = token::resolve(&mut stream).unwrap();

        assert_eq!(token, Some(Token::String("/home/user/documents")));
        let Token::String(path) = token.unwrap() else {
            panic!("expected string token");
        };
        let expected = &input[10..30];
        assert!(std::ptr::eq(path.as_ptr(), expected.as_ptr()));
    }

    #[test]
    fn tokenization_returns_none_at_end() {
        let mut stream = TokenStream::new("scope-fs");

        assert!(token::resolve(&mut stream).unwrap().is_some());
        assert_eq!(token::resolve(&mut stream).unwrap(), None);
    }

    #[test]
    fn token_resolver_recognizes_pipeline_separator() {
        let mut stream = TokenStream::new("iter |> take");
        token::resolve(&mut stream).unwrap();

        let token = token::resolve(&mut stream).unwrap();

        assert_eq!(token, Some(Token::PipelineSeparator));
    }

    #[test]
    fn token_resolver_recognizes_comma_as_separate_token() {
        let mut stream = TokenStream::new("select name, size");
        token::resolve(&mut stream).unwrap();
        token::resolve(&mut stream).unwrap();

        let token = token::resolve(&mut stream).unwrap();

        assert_eq!(token, Some(Token::Comma));
    }

    #[test]
    fn token_resolver_recognizes_parentheses_as_separate_tokens() {
        let mut stream = TokenStream::new("filter (name equals \"x\")");
        token::resolve(&mut stream).unwrap();

        let left = token::resolve(&mut stream).unwrap();
        token::resolve(&mut stream).unwrap();
        token::resolve(&mut stream).unwrap();
        token::resolve(&mut stream).unwrap();
        let right = token::resolve(&mut stream).unwrap();

        assert_eq!(left, Some(Token::LeftParen));
        assert_eq!(right, Some(Token::RightParen));
    }

    #[test]
    fn unterminated_quote_returns_tokenize_error() {
        let mut stream = TokenStream::new("scope-fs \"/tmp");
        token::resolve(&mut stream).unwrap();

        let result = token::resolve(&mut stream);

        assert!(matches!(result, Err(TokenizeError::UnterminatedString)));
    }

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
    fn parser_resolves_clear_command() {
        let mut stream = TokenStream::new("clear");

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        assert_eq!(command, Command::Clear(TerminalClearMode::Visible));
    }

    #[test]
    fn parser_resolves_clear_all_flag() {
        let mut stream = TokenStream::new("clear --all");

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        assert_eq!(command, Command::Clear(TerminalClearMode::All));
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

    #[test]
    fn tokenizer_matches_tokenize_function_pointer() {
        let tokenize: Tokenize = tokenizer::tokenize;
        let mut stream = TokenStream::new("scope-fs");

        let token = tokenize(&mut stream).unwrap();

        assert_eq!(token, Some(Token::Word("scope-fs")));
    }

    #[test]
    fn parser_matches_parse_function_pointer() {
        let parse: Parse = parser::parse;
        let mut stream = TokenStream::new("scope-fs \"/ruta\"");

        let command = parse(&mut stream, tokenizer::tokenize).unwrap();

        assert_eq!(command, Command::ScopeFs("/ruta"));
    }

    #[test]
    fn executor_matches_execute_function_pointer() {
        let directory = TestDirectory::new("execute_pointer");
        let input = format!("scope-fs \"{}\"", directory.path.display());
        let mut stream = TokenStream::new(&input);
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();
        let mut shell = shell_from_directory(&directory);
        let execute: Execute = executor::execute;

        let result = execute(&mut shell, command).unwrap();

        assert!(matches!(result, ExecutionResult::ScopeChanged));
        assert_eq!(shell.filesystem_scope().path(), directory.path.as_path());
    }

    #[test]
    fn terminal_clearer_clear_matches_use_case_function_pointer() {
        let clear: TerminalClearer = terminal_clearer::clear;

        let _ = clear;
    }

    #[test]
    fn terminal_clearer_agent_delegates_to_resolver() {
        fn resolve(mode: TerminalClearMode, _provide: Provide) -> Result<(), TerminalClearError> {
            match mode {
                TerminalClearMode::Visible => Ok(()),
                TerminalClearMode::All => Err(io::Error::other("all rejected").into()),
            }
        }

        fn provide(_mode: TerminalClearMode) -> Result<(), TerminalClearError> {
            Ok(())
        }

        let result = terminal_clearer::clear_with(TerminalClearMode::Visible, resolve, provide);
        assert!(result.is_ok());

        let result = terminal_clearer::clear_with(TerminalClearMode::All, resolve, provide);
        assert!(matches!(result, Err(TerminalClearError::Io(_))));
    }

    #[test]
    fn terminal_clearer_resolver_delegates_to_provider_and_preserves_mode() {
        fn provide(mode: TerminalClearMode) -> Result<(), TerminalClearError> {
            match mode {
                TerminalClearMode::Visible => Ok(()),
                TerminalClearMode::All => Err(io::Error::other("all rejected").into()),
            }
        }

        let resolve: Resolve = terminal_clearer_resolver::resolve;

        let result = resolve(TerminalClearMode::Visible, provide);
        assert!(result.is_ok());

        let result = resolve(TerminalClearMode::All, provide);
        assert!(matches!(result, Err(TerminalClearError::Io(_))));
    }

    #[test]
    fn terminal_clearer_provider_visible_writes_expected_ansi_sequence() {
        let mut output = Vec::new();

        terminal_clearer_provider::provide_to(&mut output, TerminalClearMode::Visible).unwrap();

        assert_eq!(output, b"\x1b[2J\x1b[H");
    }

    #[test]
    fn terminal_clearer_provider_all_writes_expected_ansi_sequence() {
        let mut output = Vec::new();

        terminal_clearer_provider::provide_to(&mut output, TerminalClearMode::All).unwrap();

        assert_eq!(output, b"\x1b[2J\x1b[3J\x1b[H");
    }

    #[test]
    fn terminal_clearer_provider_propagates_io_error() {
        let mut writer = FailingWriter;

        let result = terminal_clearer_provider::provide_to(&mut writer, TerminalClearMode::Visible);

        assert!(matches!(result, Err(TerminalClearError::Io(_))));
    }

    #[test]
    fn exiter_matches_use_case_function_pointer() {
        let exit: Exit = exiter::exit;

        let _ = exit;
    }

    #[test]
    fn shell_initializer_initialize_matches_initialize_shell_function_pointer() {
        let initialize: InitializeShell = shell_initializer::initialize;

        let shell = initialize().unwrap();

        assert!(shell.filesystem_scope().path().is_dir());
    }

    #[test]
    fn shell_resolve_initializes_shell_with_current_directory_and_real_set_scope() {
        let current_directory: CurrentDirectory = current_directory;
        let set_scope: SetFilesystemScope = scope_setter::set;
        let expected = std::env::current_dir().unwrap();

        let shell = shell::resolve(current_directory, set_scope).unwrap();

        assert_eq!(shell.filesystem_scope().path(), expected.as_path());
    }

    #[test]
    fn shell_initialized_by_resolver_owns_expected_filesystem_scope() {
        let shell = shell::resolve(current_directory, scope_setter::set).unwrap();
        let expected = std::env::current_dir().unwrap();

        assert_eq!(shell.filesystem_scope().path(), expected.as_path());
    }

    #[test]
    fn current_directory_error_produces_initialize_shell_error() {
        let result = shell::resolve(current_directory_error, scope_setter::set);

        assert!(matches!(
            result,
            Err(InitializeShellError::CurrentDirectory(_))
        ));
    }

    #[test]
    fn set_filesystem_scope_error_produces_initialize_shell_error() {
        let result = shell::resolve(current_directory, scope_error);

        assert!(matches!(result, Err(InitializeShellError::Scope(_))));
    }

    #[test]
    fn shell_cannot_be_constructed_without_filesystem_scope() {
        let shell = shell::resolve(current_directory, scope_setter::set).unwrap();

        assert!(shell.filesystem_scope().path().is_dir());
    }

    #[test]
    fn scope_fs_valid_path_replaces_previous_scope() {
        let initial = TestDirectory::new("scope_initial");
        let replacement = TestDirectory::new("scope_replacement");
        let mut shell = shell_from_directory(&initial);
        let input = format!("scope-fs \"{}\"", replacement.path.display());
        let mut stream = TokenStream::new(&input);
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = executor::execute(&mut shell, command).unwrap();

        assert!(matches!(result, ExecutionResult::ScopeChanged));
        assert_eq!(shell.filesystem_scope().path(), replacement.path.as_path());
    }

    #[test]
    fn scope_fs_invalid_path_returns_error() {
        let initial = TestDirectory::new("scope_invalid_error");
        let mut shell = shell_from_directory(&initial);
        let mut stream = TokenStream::new("scope-fs \"/definitely/not/a/directory\"");
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = executor::execute(&mut shell, command);

        assert!(matches!(result, Err(ExecuteError::Scope(_))));
    }

    #[test]
    fn scope_fs_error_leaves_previous_scope_intact() {
        let initial = TestDirectory::new("previous_scope");
        let mut shell = shell_from_directory(&initial);
        let previous_path = shell.filesystem_scope().path().to_path_buf();
        let mut stream = TokenStream::new("scope-fs \"/definitely/not/a/directory\"");
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = executor::execute(&mut shell, command);

        assert!(result.is_err());
        assert_eq!(shell.filesystem_scope().path(), previous_path.as_path());
    }

    #[test]
    fn enter_existing_child_replaces_scope_and_returns_scope_changed() {
        let directory = TestDirectory::new("enter_child_execute");
        let child = directory.path.join("child");
        fs::create_dir(&child).unwrap();
        let mut shell = shell_from_directory(&directory);

        let result = executor::execute(
            &mut shell,
            Command::Enter(CommandArgument::Literal("child")),
        )
        .unwrap();

        assert!(matches!(result, ExecutionResult::ScopeChanged));
        assert_eq!(shell.filesystem_scope().path(), child.as_path());
    }

    #[test]
    fn enter_parent_replaces_scope_with_parent_path() {
        let directory = TestDirectory::new("enter_parent_execute");
        let child = directory.path.join("child");
        fs::create_dir(&child).unwrap();
        let mut shell = shell_from_directory(&directory);
        executor::execute(
            &mut shell,
            Command::Enter(CommandArgument::Literal("child")),
        )
        .unwrap();

        let result = executor::execute(
            &mut shell,
            Command::Enter(CommandArgument::Literal("..")),
        )
        .unwrap();

        assert!(matches!(result, ExecutionResult::ScopeChanged));
        assert_eq!(
            shell.filesystem_scope().path(),
            directory.path.canonicalize().unwrap().as_path()
        );
        assert!(
            !shell
                .filesystem_scope()
                .path()
                .components()
                .any(|component| matches!(component, std::path::Component::ParentDir))
        );
    }

    #[test]
    fn enter_two_parents_replaces_scope_with_ancestor_path() {
        let directory = TestDirectory::new("enter_two_parents_execute");
        let child = directory.path.join("child");
        let grandchild = child.join("grandchild");
        fs::create_dir(&child).unwrap();
        fs::create_dir(&grandchild).unwrap();
        let mut shell = shell_from_directory(&directory);
        executor::execute(
            &mut shell,
            Command::Enter(CommandArgument::Literal("child/grandchild")),
        )
        .unwrap();

        let result = executor::execute(
            &mut shell,
            Command::Enter(CommandArgument::Literal("../..")),
        )
        .unwrap();

        assert!(matches!(result, ExecutionResult::ScopeChanged));
        assert_eq!(
            shell.filesystem_scope().path(),
            directory.path.canonicalize().unwrap().as_path()
        );
        assert!(
            !shell
                .filesystem_scope()
                .path()
                .components()
                .any(|component| matches!(component, std::path::Component::ParentDir))
        );
    }

    #[test]
    fn enter_missing_location_returns_error_and_keeps_previous_scope() {
        let directory = TestDirectory::new("enter_missing_execute");
        let mut shell = shell_from_directory(&directory);
        let previous_path = shell.filesystem_scope().path().to_path_buf();

        let result = executor::execute(
            &mut shell,
            Command::Enter(CommandArgument::Literal("missing")),
        );

        assert!(matches!(result, Err(ExecuteError::Scope(_))));
        assert_eq!(shell.filesystem_scope().path(), previous_path.as_path());
    }

    #[test]
    fn scope_fs_still_replaces_scope_after_enter_changes() {
        let initial = TestDirectory::new("scope_fs_after_enter_initial");
        let replacement = TestDirectory::new("scope_fs_after_enter_replacement");
        let mut shell = shell_from_directory(&initial);
        let input = format!("scope-fs \"{}\"", replacement.path.display());
        let mut stream = TokenStream::new(&input);
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = executor::execute(&mut shell, command).unwrap();

        assert!(matches!(result, ExecutionResult::ScopeChanged));
        assert_eq!(shell.filesystem_scope().path(), replacement.path.as_path());
    }

    #[test]
    fn iter_still_works_after_enter() {
        let directory = TestDirectory::new("iter_after_enter");
        let child = directory.path.join("child");
        fs::create_dir(&child).unwrap();
        fs::write(child.join("inside.txt"), "inside").unwrap();
        let mut shell = shell_from_directory(&directory);
        executor::execute(
            &mut shell,
            Command::Enter(CommandArgument::Literal("child")),
        )
        .unwrap();

        let result = executor::execute(&mut shell, Command::Iter).unwrap();

        assert!(matches!(result, ExecutionResult::FilesystemIteration(_)));
    }

    #[test]
    fn enter_then_iter_reads_entries_from_child_scope() {
        let directory = TestDirectory::new("enter_then_iter");
        let child = directory.path.join("child");
        fs::create_dir(&child).unwrap();
        fs::write(child.join("inside.txt"), "inside").unwrap();
        let mut shell = shell_from_directory(&directory);
        executor::execute(
            &mut shell,
            Command::Enter(CommandArgument::Literal("child")),
        )
        .unwrap();
        let result = executor::execute(&mut shell, Command::Iter).unwrap();
        let ExecutionResult::FilesystemIteration(mut iteration) = result else {
            panic!("expected filesystem iteration");
        };
        let mut found_inside = false;

        while let Some(item) = iteration_advancer::advance(&mut iteration).unwrap() {
            let entry = item.entry();
            if entry.name() == OsStr::new("inside.txt") {
                assert_eq!(entry.kind(), FilesystemEntryKind::File);
                found_inside = true;
            }
        }

        assert!(found_inside);
    }

    #[test]
    fn execute_iter_returns_filesystem_iteration() {
        let directory = TestDirectory::new("iter_execute");
        let mut shell = shell_from_directory(&directory);

        let result = executor::execute(&mut shell, Command::Iter).unwrap();

        assert!(matches!(result, ExecutionResult::FilesystemIteration(_)));
    }

    #[test]
    fn executor_delegates_pipeline_execution_and_returns_typed_pipeline_result() {
        static CALLS: AtomicUsize = AtomicUsize::new(0);

        fn clear(_mode: TerminalClearMode) -> Result<(), TerminalClearError> {
            Ok(())
        }

        fn execute_pipeline(
            shell: &Shell,
            pipeline: Pipeline,
        ) -> Result<PipelineValue, PipelineExecutionError> {
            CALLS.fetch_add(1, Ordering::SeqCst);
            assert!(shell.filesystem_scope().path().is_dir());
            assert_eq!(pipeline.operations(), &[PipelineOperation::Iter]);

            Ok(PipelineValue::Value(ProjectedValue::name("delegated.txt")))
        }

        let directory = TestDirectory::new("executor_pipeline_delegate");
        let mut shell = shell_from_directory(&directory);
        let command = Command::Pipeline(Pipeline::new(vec![PipelineOperation::Iter]));

        let result = executor::execute_with(&mut shell, command, clear, execute_pipeline).unwrap();

        assert_eq!(CALLS.load(Ordering::SeqCst), 1);
        let ExecutionResult::Pipeline(PipelineValue::Value(value)) = result else {
            panic!("expected delegated pipeline value");
        };

        assert_eq!(value, ProjectedValue::name("delegated.txt"));
    }

    #[test]
    fn executor_propagates_pipeline_execution_error() {
        fn clear(_mode: TerminalClearMode) -> Result<(), TerminalClearError> {
            Ok(())
        }

        fn execute_pipeline(
            _shell: &Shell,
            _pipeline: Pipeline,
        ) -> Result<PipelineValue, PipelineExecutionError> {
            Err(PipelineExecutionError::EmptyPipeline)
        }

        let directory = TestDirectory::new("executor_pipeline_error");
        let mut shell = shell_from_directory(&directory);
        let command = Command::Pipeline(Pipeline::new(vec![PipelineOperation::Iter]));

        let result = executor::execute_with(&mut shell, command, clear, execute_pipeline);

        assert!(matches!(
            result,
            Err(ExecuteError::Pipeline(
                PipelineExecutionError::EmptyPipeline
            ))
        ));
    }

    #[test]
    fn execute_parsed_pipeline_to_value_returns_pipeline_value() {
        let directory = TestDirectory::new("pipeline_to_value");
        fs::write(directory.path.join("only.txt"), "only").unwrap();
        let mut shell = shell_from_directory(&directory);
        let mut stream = TokenStream::new("iter |> take 1 |> select name |> to-value");
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = executor::execute(&mut shell, command).unwrap();

        let ExecutionResult::Pipeline(PipelineValue::Value(value)) = result else {
            panic!("expected typed pipeline value");
        };

        assert_eq!(value, ProjectedValue::name("only.txt"));
    }

    #[test]
    fn execute_parsed_pipeline_to_values_returns_values() {
        let directory = TestDirectory::new("pipeline_to_values");
        fs::write(directory.path.join("only.txt"), "only").unwrap();
        let mut shell = shell_from_directory(&directory);
        let mut stream = TokenStream::new("iter |> select name |> to-values");
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = executor::execute(&mut shell, command).unwrap();

        let ExecutionResult::Pipeline(PipelineValue::Values(values)) = result else {
            panic!("expected typed values");
        };

        assert_eq!(values.len(), 1);
        assert_eq!(values.items(), &[ProjectedValue::name("only.txt")]);
    }

    #[test]
    fn execute_parsed_pipeline_to_args_returns_arguments() {
        let directory = TestDirectory::new("pipeline_to_args");
        fs::write(directory.path.join("only.txt"), "only").unwrap();
        let mut shell = shell_from_directory(&directory);
        let mut stream = TokenStream::new("iter |> select name |> to-args");
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = executor::execute(&mut shell, command).unwrap();

        let ExecutionResult::Pipeline(PipelineValue::Arguments(arguments)) = result else {
            panic!("expected typed arguments");
        };

        assert_eq!(arguments.len(), 1);
        assert_eq!(arguments.items(), &[ProjectedValue::name("only.txt")]);
    }

    #[test]
    fn execute_parsed_semantically_invalid_pipeline_returns_pipeline_error() {
        let directory = TestDirectory::new("pipeline_invalid_transition");
        fs::write(directory.path.join("only.txt"), "only").unwrap();
        let mut shell = shell_from_directory(&directory);
        let mut stream = TokenStream::new("iter |> to-value");
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = executor::execute(&mut shell, command);

        assert!(matches!(
            result,
            Err(ExecuteError::Pipeline(
                PipelineExecutionError::InvalidTransition {
                    operation: PipelineOperationKind::ToValue,
                    state: PipelineValueKind::StructuredItems,
                }
            ))
        ));
    }

    #[test]
    fn execute_and_present_parsed_filter_pipeline_equals_output() {
        let directory = TestDirectory::new("pipeline_filter_equals");
        fs::write(directory.path.join("alpha.txt"), "alpha").unwrap();
        fs::write(directory.path.join("beta.txt"), "beta").unwrap();
        let mut shell = shell_from_directory(&directory);
        let mut stream = TokenStream::new(
            r#"iter |> filter name equals "alpha.txt" |> take 1 |> select name |> to-value"#,
        );
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = executor::execute(&mut shell, command).unwrap();
        let ExecutionResult::Pipeline(value) = result else {
            panic!("expected pipeline execution result");
        };

        clear_captured_output();
        pipeline_result_presenter::present_with(
            pipeline_result_presenter_resolver::resolve,
            capture_rendered,
            &shell,
            value,
        )
        .unwrap();

        let rendered = captured_rendered_output();
        assert_eq!(rendered, "alpha.txt\n");
    }

    #[test]
    fn execute_and_present_parsed_filter_pipeline_not_equals_output() {
        let directory = TestDirectory::new("pipeline_filter_not_equals");
        fs::write(directory.path.join("alpha.txt"), "alpha").unwrap();
        fs::write(directory.path.join("beta.txt"), "beta").unwrap();
        let mut shell = shell_from_directory(&directory);
        let mut stream = TokenStream::new(
            r#"iter |> filter name not-equals "alpha.txt" |> select name |> to-values"#,
        );
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = executor::execute(&mut shell, command).unwrap();
        let ExecutionResult::Pipeline(value) = result else {
            panic!("expected pipeline execution result");
        };

        clear_captured_output();
        pipeline_result_presenter::present_with(
            pipeline_result_presenter_resolver::resolve,
            capture_rendered,
            &shell,
            value,
        )
        .unwrap();

        let rendered = captured_rendered_output();
        assert_eq!(rendered, "beta.txt\n");
    }

    #[test]
    fn execute_and_present_parsed_filter_pipeline_size_decimal_output() {
        let directory = TestDirectory::new("pipeline_filter_size_decimal");
        fs::write(directory.path.join("exact.bin"), vec![b'a'; 10_000]).unwrap();
        fs::write(directory.path.join("larger.bin"), vec![b'b'; 10_240]).unwrap();
        let mut shell = shell_from_directory(&directory);
        let mut stream =
            TokenStream::new(r#"iter |> filter size equals 10kb |> select name |> to-values"#);
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();

        let result = executor::execute(&mut shell, command).unwrap();
        let ExecutionResult::Pipeline(value) = result else {
            panic!("expected pipeline execution result");
        };

        clear_captured_output();
        pipeline_result_presenter::present_with(
            pipeline_result_presenter_resolver::resolve,
            capture_rendered,
            &shell,
            value,
        )
        .unwrap();

        let rendered = captured_rendered_output();
        assert_eq!(rendered, "exact.bin\n");
        assert!(!rendered.contains("larger.bin"));
    }

    #[test]
    fn execute_clear_returns_terminal_cleared_without_changing_scope() {
        fn clear(mode: TerminalClearMode) -> Result<(), TerminalClearError> {
            match mode {
                TerminalClearMode::Visible => Ok(()),
                TerminalClearMode::All => Err(io::Error::other("expected visible").into()),
            }
        }

        let directory = TestDirectory::new("clear_execute");
        let mut shell = shell_from_directory(&directory);
        let previous_path = shell.filesystem_scope().path().to_path_buf();

        let result = execution::resolve_with(
            &mut shell,
            Command::Clear(TerminalClearMode::Visible),
            clear,
            pipeline_executor::execute,
        )
        .unwrap();

        assert!(matches!(result, ExecutionResult::TerminalCleared));
        assert_eq!(shell.filesystem_scope().path(), previous_path.as_path());
    }

    #[test]
    fn execute_clear_all_returns_terminal_cleared_without_changing_scope() {
        fn clear(mode: TerminalClearMode) -> Result<(), TerminalClearError> {
            match mode {
                TerminalClearMode::Visible => Err(io::Error::other("expected all").into()),
                TerminalClearMode::All => Ok(()),
            }
        }

        let directory = TestDirectory::new("clear_all_execute");
        let mut shell = shell_from_directory(&directory);
        let previous_path = shell.filesystem_scope().path().to_path_buf();

        let result = execution::resolve_with(
            &mut shell,
            Command::Clear(TerminalClearMode::All),
            clear,
            pipeline_executor::execute,
        )
        .unwrap();

        assert!(matches!(result, ExecutionResult::TerminalCleared));
        assert_eq!(shell.filesystem_scope().path(), previous_path.as_path());
    }

    #[test]
    fn execute_exit_returns_exit_without_changing_scope() {
        let directory = TestDirectory::new("exit_execute");
        let mut shell = shell_from_directory(&directory);
        let previous_path = shell.filesystem_scope().path().to_path_buf();

        let result = execution::resolve_with(
            &mut shell,
            Command::Exit,
            terminal_clearer::clear,
            pipeline_executor::execute,
        );

        assert!(matches!(result, Ok(ExecutionResult::Exit)));
        assert_eq!(shell.filesystem_scope().path(), previous_path.as_path());
    }

    #[test]
    fn iter_borrows_filesystem_scope_without_replacing_it() {
        let directory = TestDirectory::new("iter_borrow");
        let mut shell = shell_from_directory(&directory);
        let previous_path = shell.filesystem_scope().path().to_path_buf();

        let result = executor::execute(&mut shell, Command::Iter).unwrap();

        assert!(matches!(result, ExecutionResult::FilesystemIteration(_)));
        assert_eq!(shell.filesystem_scope().path(), previous_path.as_path());
    }

    #[test]
    fn iter_can_be_consumed_lazily_with_public_advance() {
        let directory = TestDirectory::new("iter_lazy");
        fs::write(directory.path.join("report.txt"), "report").unwrap();
        fs::create_dir(directory.path.join("images")).unwrap();
        let mut shell = shell_from_directory(&directory);

        let result = executor::execute(&mut shell, Command::Iter).unwrap();
        let ExecutionResult::FilesystemIteration(mut iteration) = result else {
            panic!("expected filesystem iteration");
        };
        let mut found_file = false;
        let mut found_directory = false;

        while let Some(item) = iteration_advancer::advance(&mut iteration).unwrap() {
            let entry = item.entry();
            if entry.name() == OsStr::new("report.txt") {
                assert_eq!(entry.kind(), FilesystemEntryKind::File);
                found_file = true;
            }

            if entry.name() == OsStr::new("images") {
                assert_eq!(entry.kind(), FilesystemEntryKind::Directory);
                found_directory = true;
            }
        }

        assert!(found_file);
        assert!(found_directory);
    }

    #[test]
    fn iter_error_is_converted_to_execute_error() {
        let directory = TestDirectory::new("iter_error");
        let mut shell = shell_from_directory(&directory);
        fs::remove_dir_all(&directory.path).unwrap();

        let result = executor::execute(&mut shell, Command::Iter);

        assert!(matches!(result, Err(ExecuteError::Iter(_))));
    }

    #[test]
    fn advance_errors_remain_iter_error() {
        let directory = TestDirectory::new("advance_error");
        let mut shell = shell_from_directory(&directory);
        fs::write(directory.path.join("report.txt"), "report").unwrap();
        let result = executor::execute(&mut shell, Command::Iter).unwrap();
        let ExecutionResult::FilesystemIteration(mut iteration) = result else {
            panic!("expected filesystem iteration");
        };
        fs::remove_dir_all(&directory.path).unwrap();

        let result = iteration_advancer::advance(&mut iteration);

        if let Err(error) = result {
            assert!(matches!(
                error,
                IterError::NextEntry(_) | IterError::MaterializeEntry(_)
            ));
        }
    }
}
