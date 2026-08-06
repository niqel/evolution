use std::fmt;
use std::io::{self, Write};
use std::path::{Component, Path};

use evo_shell::{
    ExecuteError, ExecutionResult, ParseError, PipelineResultPresentError, PresentPipelineResult,
    Shell, StartError, Token, TokenStream, TokenizeError, executor, iteration_presenter, parser,
    pipeline_result_presenter, presentation_style, starter, tokenizer,
};
use evo_shell_engine::IterError;

enum LoopControl {
    Continue,
    Exit,
}

#[derive(Debug)]
pub enum ReadInputError {
    Io(io::Error),
    Tokenize(TokenizeError),
}

impl From<io::Error> for ReadInputError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<TokenizeError> for ReadInputError {
    fn from(error: TokenizeError) -> Self {
        Self::Tokenize(error)
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), RunError> {
    let mut shell = starter::start()?;
    run_loop(&mut shell)?;
    Ok(())
}

fn run_loop(shell: &mut Shell) -> Result<(), RunError> {
    loop {
        write_prompt(shell)?;

        let input = match read_input() {
            Ok(Some(input)) => input,
            Ok(None) => {
                println!();
                return Ok(());
            }
            Err(ReadInputError::Tokenize(error)) => {
                render_parse_error(ParseError::Tokenize(error));
                continue;
            }
            Err(ReadInputError::Io(error)) => return Err(RunError::Io(error)),
        };

        match handle_input(shell, &input)? {
            LoopControl::Continue => {}
            LoopControl::Exit => return Ok(()),
        }
    }
}

fn write_prompt(shell: &Shell) -> io::Result<()> {
    let mut stdout = io::stdout();
    write_prompt_to(&mut stdout, shell.filesystem_scope().path())?;
    stdout.flush()
}

fn write_prompt_to(writer: &mut impl Write, path: &Path) -> io::Result<()> {
    write!(
        writer,
        "{}scope-fs{} {}{}{} {}>{} {}",
        presentation_style::PROMPT_SCOPE_STYLE,
        presentation_style::RESET,
        presentation_style::PROMPT_LOCATION_STYLE,
        compact_scope_location(path),
        presentation_style::RESET,
        presentation_style::PROMPT_SCOPE_STYLE,
        presentation_style::RESET,
        presentation_style::FILE_STYLE,
    )
}

fn write_continuation_prompt_to(writer: &mut impl Write) -> io::Result<()> {
    write!(
        writer,
        "{}...{} {}>{} {}",
        presentation_style::PROMPT_SCOPE_STYLE,
        presentation_style::RESET,
        presentation_style::PROMPT_SCOPE_STYLE,
        presentation_style::RESET,
        presentation_style::FILE_STYLE,
    )
}

fn reset_after_input_to(writer: &mut impl Write) -> io::Result<()> {
    write!(writer, "{}", presentation_style::RESET)
}

fn read_input() -> Result<Option<String>, ReadInputError> {
    let mut stdin = io::stdin().lock();
    let mut stdout = io::stdout();
    read_input_from(&mut stdin, &mut stdout)
}

fn read_input_from(
    reader: &mut impl io::BufRead,
    writer: &mut impl Write,
) -> Result<Option<String>, ReadInputError> {
    let mut accumulated = String::new();
    let bytes_read = reader.read_line(&mut accumulated)?;

    if bytes_read == 0 {
        return Ok(None);
    }

    reset_after_input_to(writer)?;
    writer.flush()?;

    while requires_continuation(&accumulated)? {
        write_continuation_prompt_to(writer)?;
        writer.flush()?;
        let mut line = String::new();
        let bytes = reader.read_line(&mut line)?;
        if bytes == 0 {
            return Ok(None);
        }
        reset_after_input_to(writer)?;
        writer.flush()?;
        accumulated.push_str(&line);
    }

    Ok(Some(accumulated))
}

fn requires_continuation(input: &str) -> Result<bool, TokenizeError> {
    let mut stream = TokenStream::new(input);
    let mut last_token = None;
    let mut open_parens: usize = 0;

    while let Some(token) = tokenizer::tokenize(&mut stream)? {
        match token {
            Token::LeftParen => open_parens += 1,
            Token::RightParen => {
                open_parens = open_parens.saturating_sub(1);
            }
            _ => {}
        }
        last_token = Some(token);
    }

    Ok(open_parens > 0 || matches!(last_token, Some(Token::PipelineSeparator)))
}

fn handle_input(shell: &mut Shell, input: &str) -> io::Result<LoopControl> {
    if input.trim().is_empty() {
        return Ok(LoopControl::Continue);
    }

    let mut stream = TokenStream::new(input);
    let command = match parser::parse(&mut stream, tokenizer::tokenize) {
        Ok(command) => command,
        Err(error) => {
            render_parse_error(error);
            return Ok(LoopControl::Continue);
        }
    };

    match executor::execute(shell, command) {
        Ok(result) => render_execution(shell, result),
        Err(error) => {
            render_execute_error(error);
            Ok(LoopControl::Continue)
        }
    }
}

fn render_execution(shell: &Shell, result: ExecutionResult) -> io::Result<LoopControl> {
    render_execution_with(shell, result, pipeline_result_presenter::present)
}

fn render_execution_with(
    shell: &Shell,
    result: ExecutionResult,
    present_pipeline_result: PresentPipelineResult,
) -> io::Result<LoopControl> {
    match result {
        ExecutionResult::ScopeChanged => {
            render_scope_changed(&mut io::stdout())?;
            Ok(LoopControl::Continue)
        }
        ExecutionResult::TerminalCleared => Ok(LoopControl::Continue),
        ExecutionResult::FilesystemIteration(iteration) => {
            match iteration_presenter::present(iteration) {
                Ok(()) => Ok(LoopControl::Continue),
                Err(iteration_presenter::PresentIterationError::Io(error)) => Err(error),
                Err(iteration_presenter::PresentIterationError::Iter(error)) => {
                    render_iter_error(error);
                    Ok(LoopControl::Continue)
                }
            }
        }
        ExecutionResult::Pipeline(pipeline) => {
            present_pipeline_result(shell, pipeline).map_err(|error| match error {
                PipelineResultPresentError::Io(error) => error,
            })?;
            Ok(LoopControl::Continue)
        }
        ExecutionResult::Exit => Ok(LoopControl::Exit),
    }
}

fn render_scope_changed(_writer: &mut impl Write) -> io::Result<()> {
    Ok(())
}

fn compact_scope_location(path: &Path) -> String {
    let mut normal_count = 0;
    let mut last_normal = None;

    for component in path.components() {
        if let Component::Normal(name) = component {
            normal_count += 1;
            last_normal = Some(name);
        }
    }

    match (normal_count, last_normal) {
        (0, _) => path.display().to_string(),
        (1, Some(only)) => {
            if path.is_absolute() {
                path.display().to_string()
            } else {
                only.to_string_lossy().into_owned()
            }
        }
        (_, Some(last)) => {
            format!("…/{}", last.to_string_lossy())
        }
        _ => path.display().to_string(),
    }
}

fn render_parse_error(error: ParseError<'_>) {
    eprintln!("{error:?}");
}

fn render_execute_error(error: ExecuteError) {
    eprintln!("{error:?}");
}

fn render_iter_error(error: IterError) {
    eprintln!("{error:?}");
}

#[derive(Debug)]
enum RunError {
    Start(StartError),
    Io(io::Error),
}

impl fmt::Display for RunError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Start(error) => write!(formatter, "{error:?}"),
            Self::Io(error) => write!(formatter, "{error}"),
        }
    }
}

impl From<StartError> for RunError {
    fn from(error: StartError) -> Self {
        Self::Start(error)
    }
}

impl From<io::Error> for RunError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::path::Path;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::{
        LoopControl, ReadInputError, compact_scope_location, presentation_style, read_input_from,
        render_execution, render_execution_with, render_scope_changed, requires_continuation,
        reset_after_input_to, write_prompt_to,
    };
    use evo_shell::{
        Command, ExecutionResult, PipelineResultPresentError, PipelineValue, TokenStream, executor,
        parser, shell_initializer, tokenizer,
    };
    use evo_shell_engine::{ProjectedValue, SelectProperty};

    #[test]
    fn compact_scope_location_uses_last_segment_for_deep_path() {
        let path = Path::new("/home/user/repos/evolution/evo-shell/src");

        let result = compact_scope_location(path);

        assert_eq!(result, "…/src");
    }

    #[test]
    fn compact_scope_location_uses_last_segment_after_enter_agents() {
        let path = Path::new("/home/user/repos/evolution/evo-shell/src/agents");

        let result = compact_scope_location(path);

        assert_eq!(result, "…/agents");
    }

    #[test]
    fn compact_scope_location_uses_resolved_parent_path() {
        let path = Path::new("/home/user/repos/evolution/evo-shell");

        let result = compact_scope_location(path);

        assert_eq!(result, "…/evo-shell");
    }

    #[cfg(unix)]
    #[test]
    fn compact_scope_location_represents_unix_root() {
        let result = compact_scope_location(Path::new("/"));

        assert_eq!(result, "/");
    }

    #[test]
    fn write_prompt_uses_scope_type_and_compact_location_with_distinct_styles() {
        let mut output = Vec::new();

        write_prompt_to(
            &mut output,
            Path::new("/home/user/repos/evolution/evo-shell/src"),
        )
        .unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            format!(
                "{}scope-fs{} {}…/src{} {}>{} {}",
                presentation_style::PROMPT_SCOPE_STYLE,
                presentation_style::RESET,
                presentation_style::PROMPT_LOCATION_STYLE,
                presentation_style::RESET,
                presentation_style::PROMPT_SCOPE_STYLE,
                presentation_style::RESET,
                presentation_style::FILE_STYLE
            )
        );
        assert_ne!(
            presentation_style::PROMPT_SCOPE_STYLE,
            presentation_style::PROMPT_LOCATION_STYLE
        );
    }

    #[test]
    fn write_prompt_styles_separator_and_activates_input_style() {
        let mut output = Vec::new();

        write_prompt_to(
            &mut output,
            Path::new("/home/user/repos/evolution/evo-shell/src"),
        )
        .unwrap();

        let output = String::from_utf8(output).unwrap();
        assert!(output.contains(&format!(
            "{}>{} {}",
            presentation_style::PROMPT_SCOPE_STYLE,
            presentation_style::RESET,
            presentation_style::FILE_STYLE
        )));
        assert!(output.ends_with(presentation_style::FILE_STYLE));
        assert!(!output.contains("/home/user/repos/evolution/evo-shell/src"));
    }

    #[test]
    fn reset_after_input_writes_reset() {
        let mut output = Vec::new();

        reset_after_input_to(&mut output).unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            presentation_style::RESET
        );
    }

    #[test]
    fn scope_changed_does_not_render_redundant_active_scope_line() {
        let mut output = Vec::new();

        render_scope_changed(&mut output).unwrap();

        assert!(output.is_empty());
    }

    #[test]
    fn render_execution_returns_exit_control_for_exit_result() {
        let shell = shell_initializer::initialize().unwrap();

        let result = render_execution(&shell, ExecutionResult::Exit).unwrap();

        assert!(matches!(result, LoopControl::Exit));
    }

    #[test]
    fn render_execution_delegates_pipeline_results_to_presenter() {
        static CALLS: AtomicUsize = AtomicUsize::new(0);
        let shell = shell_initializer::initialize().unwrap();

        let result = render_execution_with(
            &shell,
            ExecutionResult::Pipeline(PipelineValue::Value(ProjectedValue::name("only.txt"))),
            |shell, value| {
                assert!(shell.filesystem_scope().path().is_dir());
                CALLS.fetch_add(1, Ordering::SeqCst);
                assert!(matches!(value, PipelineValue::Value(_)));
                Ok(())
            },
        )
        .unwrap();

        assert!(matches!(result, LoopControl::Continue));
        assert_eq!(CALLS.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn render_execution_propagates_pipeline_presenter_error_as_io_error() {
        let shell = shell_initializer::initialize().unwrap();
        let result = render_execution_with(
            &shell,
            ExecutionResult::Pipeline(PipelineValue::Value(ProjectedValue::name("only.txt"))),
            |_shell, _value| {
                Err(PipelineResultPresentError::Io(std::io::Error::other(
                    "boom",
                )))
            },
        );

        assert!(matches!(result, Err(error) if error.kind() == std::io::ErrorKind::Other));
    }

    #[test]
    fn vertical_pipeline_parse_execute_and_present_writes_only_typed_value() {
        thread_local! {
            static CAPTURED: RefCell<String> = const { RefCell::new(String::new()) };
        }

        fn present_for_test(
            _shell: &evo_shell::Shell,
            value: PipelineValue,
        ) -> Result<(), PipelineResultPresentError> {
            CAPTURED.with(|captured| {
                let mut captured = captured.borrow_mut();
                captured.clear();

                if let PipelineValue::Value(ProjectedValue::Name(name)) = value {
                    captured.push_str(&name.to_string_lossy());
                    captured.push('\n');
                }
            });

            Ok(())
        }

        let directory = std::env::temp_dir().join(format!(
            "evo_shell_vertical_pipeline_present_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&directory).unwrap();
        std::fs::write(directory.join("only.txt"), "only").unwrap();

        let mut shell = shell_initializer::initialize().unwrap();
        executor::execute(
            &mut shell,
            Command::ScopeFs(directory.to_str().expect("temp path should be utf-8")),
        )
        .unwrap();

        let mut stream = TokenStream::new("iter |> take 1 |> select name |> to-value");
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();
        let result = executor::execute(&mut shell, command).unwrap();

        let loop_control = render_execution_with(&shell, result, present_for_test).unwrap();
        let rendered = CAPTURED.with(|captured| captured.borrow().clone());

        assert!(matches!(loop_control, LoopControl::Continue));
        assert_eq!(rendered, "only.txt\n");

        let _ = std::fs::remove_dir_all(&directory);
    }

    #[test]
    fn single_line_input_without_pipeline_separator_does_not_request_continuation() {
        let mut input = std::io::Cursor::new(b"iter\n");
        let mut output = Vec::new();

        let result = read_input_from(&mut input, &mut output).unwrap();

        assert_eq!(result.as_deref(), Some("iter\n"));
        assert!(output.is_empty() || output == presentation_style::RESET.as_bytes());
    }

    #[test]
    fn line_ending_with_pipeline_separator_requests_continuation() {
        let mut input = std::io::Cursor::new(b"iter |>\ntake 1\n");
        let mut output = Vec::new();

        let result = read_input_from(&mut input, &mut output).unwrap();

        assert_eq!(result.as_deref(), Some("iter |>\ntake 1\n"));
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("..."));
    }

    #[test]
    fn multiline_pipeline_combines_multiple_lines_into_single_input() {
        let mut input =
            std::io::Cursor::new(b"iter |>\n    take 1 |>\n    select name |>\n    to-value\n");
        let mut output = Vec::new();

        let result = read_input_from(&mut input, &mut output).unwrap();

        assert_eq!(
            result.as_deref(),
            Some("iter |>\n    take 1 |>\n    select name |>\n    to-value\n")
        );
    }

    #[test]
    fn trailing_whitespace_after_pipeline_separator_still_requires_continuation() {
        let mut input = std::io::Cursor::new(b"iter |>    \ntake 1\n");
        let mut output = Vec::new();

        let result = read_input_from(&mut input, &mut output).unwrap();

        assert_eq!(result.as_deref(), Some("iter |>    \ntake 1\n"));
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("..."));
    }

    #[test]
    fn pipeline_separator_inside_quoted_string_does_not_trigger_continuation() {
        let mut input = std::io::Cursor::new(b"filter name equals \"foo |> bar\"\n");
        let mut output = Vec::new();

        let result = read_input_from(&mut input, &mut output).unwrap();

        assert_eq!(
            result.as_deref(),
            Some("filter name equals \"foo |> bar\"\n")
        );
        let output_str = String::from_utf8(output).unwrap();
        assert!(!output_str.contains("..."));
    }

    #[test]
    fn complete_multiline_input_parses_as_command_pipeline() {
        use evo_shell::{Command, PipelineOperation, TokenStream, parser, tokenizer};

        let multiline = "iter |>\n    take 1 |>\n    select name |>\n    to-value";
        let mut stream = TokenStream::new(multiline);

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
    fn vertical_multiline_pipeline_parse_execute_and_present_writes_only_typed_value() {
        use evo_shell::{
            Command, PipelineResultPresentError, PipelineValue, executor, parser,
            shell_initializer, tokenizer,
        };

        thread_local! {
            static CAPTURED: RefCell<String> = const { RefCell::new(String::new()) };
        }

        fn present_for_test(
            _shell: &evo_shell::Shell,
            value: PipelineValue,
        ) -> Result<(), PipelineResultPresentError> {
            CAPTURED.with(|captured| {
                let mut captured = captured.borrow_mut();
                captured.clear();

                if let PipelineValue::Value(ProjectedValue::Name(name)) = value {
                    captured.push_str(&name.to_string_lossy());
                    captured.push('\n');
                }
            });

            Ok(())
        }

        let directory = std::env::temp_dir().join(format!(
            "evo_shell_multiline_vertical_pipeline_present_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&directory).unwrap();
        std::fs::write(directory.join("only.txt"), "only").unwrap();

        let mut shell = shell_initializer::initialize().unwrap();
        executor::execute(
            &mut shell,
            Command::ScopeFs(directory.to_str().expect("temp path should be utf-8")),
        )
        .unwrap();

        let mut input =
            std::io::Cursor::new(b"iter |>\n    take 1 |>\n    select name |>\n    to-value\n");
        let mut output = Vec::new();
        let multiline_input = read_input_from(&mut input, &mut output).unwrap().unwrap();

        let mut stream = TokenStream::new(&multiline_input);
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();
        let result = executor::execute(&mut shell, command).unwrap();

        let loop_control = render_execution_with(&shell, result, present_for_test).unwrap();
        let rendered = CAPTURED.with(|captured| captured.borrow().clone());

        assert!(matches!(loop_control, LoopControl::Continue));
        assert_eq!(rendered, "only.txt\n");

        let _ = std::fs::remove_dir_all(&directory);
    }

    #[test]
    fn simple_commands_continue_working_without_multiline() {
        use evo_shell::{Command, TokenStream, parser, tokenizer};

        let mut input = std::io::Cursor::new(b"exit\n");
        let mut output = Vec::new();

        let result = read_input_from(&mut input, &mut output).unwrap();

        assert_eq!(result.as_deref(), Some("exit\n"));

        let mut stream = TokenStream::new("exit");
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();
        assert_eq!(command, Command::Exit);
    }

    #[test]
    fn multiline_filter_pipeline_produces_identical_result_to_single_line() {
        use evo_shell::{Command, TokenStream, executor, parser, shell_initializer, tokenizer};

        let directory = std::env::temp_dir().join(format!(
            "evo_shell_multiline_filter_test_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&directory).unwrap();
        std::fs::write(directory.join("alpha.txt"), "alpha").unwrap();
        std::fs::write(directory.join("beta.txt"), "beta").unwrap();

        let mut shell_single = shell_initializer::initialize().unwrap();
        executor::execute(
            &mut shell_single,
            Command::ScopeFs(directory.to_str().expect("temp path should be utf-8")),
        )
        .unwrap();

        let mut shell_multi = shell_initializer::initialize().unwrap();
        executor::execute(
            &mut shell_multi,
            Command::ScopeFs(directory.to_str().expect("temp path should be utf-8")),
        )
        .unwrap();

        let single_line = r#"iter |> filter name equals "alpha.txt" |> select name |> to-values"#;
        let mut input = std::io::Cursor::new(
            b"iter |>\nfilter name equals \"alpha.txt\" |>\nselect name |>\nto-values\n",
        );
        let mut output = Vec::new();
        let multiline = read_input_from(&mut input, &mut output).unwrap().unwrap();

        let mut stream_single = TokenStream::new(single_line);
        let cmd_single = parser::parse(&mut stream_single, tokenizer::tokenize).unwrap();
        let res_single = executor::execute(&mut shell_single, cmd_single).unwrap();

        let mut stream_multi = TokenStream::new(&multiline);
        let cmd_multi = parser::parse(&mut stream_multi, tokenizer::tokenize).unwrap();
        let res_multi = executor::execute(&mut shell_multi, cmd_multi).unwrap();

        assert_eq!(format!("{res_single:?}"), format!("{res_multi:?}"));

        let _ = std::fs::remove_dir_all(&directory);
    }

    #[test]
    fn continuation_detection_propagates_unterminated_quote_error() {
        use evo_shell::TokenizeError;

        let input = "iter |> \"unterminated";
        let result = requires_continuation(input);

        assert_eq!(result, Err(TokenizeError::UnterminatedString));
    }

    #[test]
    fn standalone_unterminated_quote_is_not_treated_as_complete_input() {
        use evo_shell::TokenizeError;

        let input = "\"unterminated";
        let result = requires_continuation(input);

        assert_eq!(result, Err(TokenizeError::UnterminatedString));
    }

    #[test]
    fn valid_trailing_pipeline_separator_still_requests_continuation() {
        let input = "iter |>";
        let result = requires_continuation(input);

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn quoted_pipeline_separator_still_does_not_request_continuation() {
        let input = "filter name equals \"foo |> bar\"";
        let result = requires_continuation(input);

        assert_eq!(result, Ok(false));
    }

    #[test]
    fn read_input_from_propagates_tokenize_error() {
        use evo_shell::TokenizeError;

        let mut input = std::io::Cursor::new(b"iter |>\n\"unterminated\n");
        let mut output = Vec::new();

        let result = read_input_from(&mut input, &mut output);

        assert!(matches!(
            result,
            Err(ReadInputError::Tokenize(TokenizeError::UnterminatedString))
        ));
    }

    #[test]
    fn open_parenthesis_requests_continuation() {
        let input = "(\niter |> take 1";
        let result = requires_continuation(input);

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn balanced_parentheses_complete_input() {
        let input = "(iter |> take 1)";
        let result = requires_continuation(input);

        assert_eq!(result, Ok(false));
    }

    #[test]
    fn parenthesis_inside_quoted_string_does_not_open_group() {
        let input = "filter name equals \"(README)\"";
        let result = requires_continuation(input);

        assert_eq!(result, Ok(false));
    }

    #[test]
    fn filter_parentheses_continue_working() {
        use evo_shell::{Command, TokenStream, parser, tokenizer};

        let input = r#"iter |> filter (name equals "a" or name equals "b")"#;
        let mut stream = TokenStream::new(input);

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();
        assert!(matches!(command, Command::Pipeline(_)));
    }

    #[test]
    fn grouped_pipeline_parses() {
        use evo_shell::{Command, TokenStream, parser, tokenizer};

        let input = "(iter |> take 1 |> select name |> to-value)";
        let mut stream = TokenStream::new(input);

        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();
        assert!(matches!(command, Command::Grouped(_)));
    }

    #[test]
    fn grouped_pipeline_evaluates_to_same_value_as_inner_pipeline() {
        let directory = std::env::temp_dir().join(format!(
            "evo_shell_grouped_eval_test_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&directory).unwrap();
        std::fs::write(directory.join("only.txt"), "only").unwrap();

        let mut shell_grouped = shell_initializer::initialize().unwrap();
        executor::execute(
            &mut shell_grouped,
            Command::ScopeFs(directory.to_str().expect("temp path should be utf-8")),
        )
        .unwrap();

        let mut shell_inner = shell_initializer::initialize().unwrap();
        executor::execute(
            &mut shell_inner,
            Command::ScopeFs(directory.to_str().expect("temp path should be utf-8")),
        )
        .unwrap();

        let mut stream_grouped = TokenStream::new("(iter |> take 1 |> select name |> to-value)");
        let cmd_grouped = parser::parse(&mut stream_grouped, tokenizer::tokenize).unwrap();
        let res_grouped = executor::execute(&mut shell_grouped, cmd_grouped).unwrap();

        let mut stream_inner = TokenStream::new("iter |> take 1 |> select name |> to-value");
        let cmd_inner = parser::parse(&mut stream_inner, tokenizer::tokenize).unwrap();
        let res_inner = executor::execute(&mut shell_inner, cmd_inner).unwrap();

        assert_eq!(format!("{res_grouped:?}"), format!("{res_inner:?}"));

        let _ = std::fs::remove_dir_all(&directory);
    }

    #[test]
    fn multiline_grouped_pipeline_works() {
        let input_text = "(\n    iter |>\n    take 1 |>\n    select name |>\n    to-value\n)";
        let mut input = std::io::Cursor::new(input_text.as_bytes());
        let mut output = Vec::new();

        let result = read_input_from(&mut input, &mut output).unwrap();

        assert_eq!(result.as_deref(), Some(input_text));
    }

    #[test]
    fn unmatched_closing_parenthesis_returns_parse_error() {
        use evo_shell::{ParseError, TokenStream, parser, tokenizer};

        let mut stream = TokenStream::new(")");
        let result = parser::parse(&mut stream, tokenizer::tokenize);

        assert_eq!(result, Err(ParseError::UnexpectedClosingParenthesis));
    }

    #[test]
    fn eof_with_open_group_does_not_execute_partial_expression() {
        let mut input = std::io::Cursor::new(b"(\niter |> take 1");
        let mut output = Vec::new();

        let result = read_input_from(&mut input, &mut output).unwrap();

        assert_eq!(result, None);
    }

    #[test]
    fn vertical_grouped_pipeline_test() {
        thread_local! {
            static CAPTURED: RefCell<String> = const { RefCell::new(String::new()) };
        }

        fn present_for_test(
            _shell: &evo_shell::Shell,
            value: PipelineValue,
        ) -> Result<(), PipelineResultPresentError> {
            CAPTURED.with(|captured| {
                let mut captured = captured.borrow_mut();
                captured.clear();

                if let PipelineValue::Value(ProjectedValue::Name(name)) = value {
                    captured.push_str(&name.to_string_lossy());
                    captured.push('\n');
                }
            });

            Ok(())
        }

        let directory = std::env::temp_dir().join(format!(
            "evo_shell_vertical_grouped_test_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&directory).unwrap();
        std::fs::write(directory.join("only.txt"), "only").unwrap();

        let mut shell = shell_initializer::initialize().unwrap();
        executor::execute(
            &mut shell,
            Command::ScopeFs(directory.to_str().expect("temp path should be utf-8")),
        )
        .unwrap();

        let multiline_input = "(\n    iter |>\n    take 1 |>\n    select name |>\n    to-value\n)";
        let mut stream = TokenStream::new(multiline_input);
        let command = parser::parse(&mut stream, tokenizer::tokenize).unwrap();
        let result = executor::execute(&mut shell, command).unwrap();

        let loop_control = render_execution_with(&shell, result, present_for_test).unwrap();
        let rendered = CAPTURED.with(|captured| captured.borrow().clone());

        assert!(matches!(loop_control, LoopControl::Continue));
        assert_eq!(rendered, "only.txt\n");

        let _ = std::fs::remove_dir_all(&directory);
    }
}
