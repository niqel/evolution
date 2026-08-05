use std::fmt;
use std::io::{self, Write};
use std::path::{Component, Path};

use evo_shell::{
    ExecuteError, ExecutionResult, ParseError, Shell, StartError, TokenStream, executor,
    iteration_presenter, parser, presentation_style, starter, tokenizer,
};
use evo_shell_engine::IterError;

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

        let input = read_input();
        reset_after_input()?;

        let Some(input) = input? else {
            println!();
            return Ok(());
        };

        handle_input(shell, &input)?;
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

fn reset_after_input() -> io::Result<()> {
    let mut stdout = io::stdout();
    reset_after_input_to(&mut stdout)?;
    stdout.flush()
}

fn reset_after_input_to(writer: &mut impl Write) -> io::Result<()> {
    write!(writer, "{}", presentation_style::RESET)
}

fn read_input() -> io::Result<Option<String>> {
    let mut input = String::new();
    let bytes_read = io::stdin().read_line(&mut input)?;

    if bytes_read == 0 {
        return Ok(None);
    }

    Ok(Some(input))
}

fn handle_input(shell: &mut Shell, input: &str) -> io::Result<()> {
    if input.trim().is_empty() {
        return Ok(());
    }

    let mut stream = TokenStream::new(input);
    let command = match parser::parse(&mut stream, tokenizer::tokenize) {
        Ok(command) => command,
        Err(error) => {
            render_parse_error(error);
            return Ok(());
        }
    };

    match executor::execute(shell, command) {
        Ok(result) => render_execution(shell, result),
        Err(error) => {
            render_execute_error(error);
            Ok(())
        }
    }
}

fn render_execution(_shell: &Shell, result: ExecutionResult) -> io::Result<()> {
    match result {
        ExecutionResult::ScopeChanged => render_scope_changed(&mut io::stdout()),
        ExecutionResult::TerminalCleared => Ok(()),
        ExecutionResult::FilesystemIteration(iteration) => {
            match iteration_presenter::present(iteration) {
                Ok(()) => Ok(()),
                Err(iteration_presenter::PresentIterationError::Io(error)) => Err(error),
                Err(iteration_presenter::PresentIterationError::Iter(error)) => {
                    render_iter_error(error);
                    Ok(())
                }
            }
        }
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
    use std::path::Path;

    use super::{
        compact_scope_location, presentation_style, render_scope_changed, reset_after_input_to,
        write_prompt_to,
    };

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
}
