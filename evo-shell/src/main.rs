use std::fmt;
use std::io::{self, Write};
use std::path::{Component, Path};

use evo_shell::{
    ExecuteError, ExecutionResult, InitializeShellError, ParseError, Shell, TokenStream, executor,
    iteration_presenter, parser, shell_initializer, tokenizer,
};
use evo_shell_engine::IterError;

const PROMPT_SCOPE_COLOR: &str = "\x1b[32m";
const PROMPT_LOCATION_COLOR: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), RunError> {
    let mut shell = shell_initializer::initialize()?;
    run_loop(&mut shell)?;
    Ok(())
}

fn run_loop(shell: &mut Shell) -> Result<(), RunError> {
    loop {
        write_prompt(shell)?;

        let Some(input) = read_input()? else {
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
        "{PROMPT_SCOPE_COLOR}scope-fs{RESET} {PROMPT_LOCATION_COLOR}{}{RESET} > ",
        compact_scope_location(path)
    )
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
    InitializeShell(InitializeShellError),
    Io(io::Error),
}

impl fmt::Display for RunError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InitializeShell(error) => write!(formatter, "{error:?}"),
            Self::Io(error) => write!(formatter, "{error}"),
        }
    }
}

impl From<InitializeShellError> for RunError {
    fn from(error: InitializeShellError) -> Self {
        Self::InitializeShell(error)
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
        PROMPT_LOCATION_COLOR, PROMPT_SCOPE_COLOR, RESET, compact_scope_location,
        render_scope_changed, write_prompt_to,
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
            format!("{PROMPT_SCOPE_COLOR}scope-fs{RESET} {PROMPT_LOCATION_COLOR}…/src{RESET} > ")
        );
        assert_ne!(PROMPT_SCOPE_COLOR, PROMPT_LOCATION_COLOR);
    }

    #[test]
    fn write_prompt_keeps_separator_neutral_after_reset() {
        let mut output = Vec::new();

        write_prompt_to(
            &mut output,
            Path::new("/home/user/repos/evolution/evo-shell/src"),
        )
        .unwrap();

        let output = String::from_utf8(output).unwrap();
        assert!(output.ends_with(&format!("{RESET} > ")));
        assert!(!output.contains("/home/user/repos/evolution/evo-shell/src"));
    }

    #[test]
    fn scope_changed_does_not_render_redundant_active_scope_line() {
        let mut output = Vec::new();

        render_scope_changed(&mut output).unwrap();

        assert!(output.is_empty());
    }
}
