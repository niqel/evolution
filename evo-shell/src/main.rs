use std::fmt;
use std::io::{self, Write};

use evo_shell::{
    ExecuteError, ExecutionResult, InitializeShellError, ParseError, Shell, TokenStream, executor,
    parser, shell_initializer, tokenizer,
};
use evo_shell_engine::{
    FilesystemEntry, FilesystemEntryKind, FilesystemIteration, IterError, iteration_advancer,
};

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
    write!(stdout, "{} > ", shell.filesystem_scope().path().display())?;
    stdout.flush()
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

fn render_execution(shell: &Shell, result: ExecutionResult) -> io::Result<()> {
    match result {
        ExecutionResult::ScopeChanged => {
            let mut stdout = io::stdout();
            writeln!(
                stdout,
                "Scope activo: {}",
                shell.filesystem_scope().path().display()
            )
        }
        ExecutionResult::FilesystemIteration(iteration) => render_iteration(iteration),
    }
}

fn render_iteration(mut iteration: FilesystemIteration) -> io::Result<()> {
    loop {
        match iteration_advancer::advance(&mut iteration) {
            Ok(Some(entry)) => render_entry(&entry)?,
            Ok(None) => return Ok(()),
            Err(error) => {
                render_iter_error(error);
                return Ok(());
            }
        }
    }
}

fn render_entry(entry: &FilesystemEntry) -> io::Result<()> {
    let mut stdout = io::stdout();
    writeln!(stdout, "{}", entry_display_name(entry))
}

fn entry_display_name(entry: &FilesystemEntry) -> String {
    let name = entry.name().to_string_lossy();

    match entry.kind() {
        FilesystemEntryKind::Directory => format!("{name}/"),
        FilesystemEntryKind::Symlink => format!("{name}@"),
        FilesystemEntryKind::File | FilesystemEntryKind::Other => name.into_owned(),
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
