use std::path::Path;

use evo_shell_engine::{Enter, Iter, SetFilesystemScope, enterer, iterator, scope_setter};

use crate::agents::exiter;
use crate::definitions::domain::entities::command::Command;
use crate::definitions::domain::entities::shell::Shell;
use crate::definitions::use_cases::execute::{ExecuteError, ExecutionResult};
use crate::definitions::use_cases::exiter::Exit;
use crate::definitions::use_cases::terminal_clearer::TerminalClearer;
use crate::terminal_clearer;

pub fn resolve(shell: &mut Shell, command: Command<'_>) -> Result<ExecutionResult, ExecuteError> {
    let clear: TerminalClearer = terminal_clearer::clear;

    resolve_with(shell, command, clear)
}

pub(crate) fn resolve_with(
    shell: &mut Shell,
    command: Command<'_>,
    clear: TerminalClearer,
) -> Result<ExecutionResult, ExecuteError> {
    match command {
        Command::ScopeFs(path) => {
            let set_scope: SetFilesystemScope = scope_setter::set;
            let filesystem_scope = set_scope(Path::new(path)).map_err(ExecuteError::Scope)?;
            shell.replace_filesystem_scope(filesystem_scope);
            Ok(ExecutionResult::ScopeChanged)
        }
        Command::Iter => {
            let iter: Iter = iterator::iter;
            let iteration = iter(shell.filesystem_scope()).map_err(ExecuteError::Iter)?;
            Ok(ExecutionResult::FilesystemIteration(iteration))
        }
        Command::Enter(location) => {
            let enter: Enter = enterer::enter;
            let filesystem_scope = enter(shell.filesystem_scope(), Path::new(location))
                .map_err(ExecuteError::Scope)?;
            shell.replace_filesystem_scope(filesystem_scope);
            Ok(ExecutionResult::ScopeChanged)
        }
        Command::Clear(mode) => {
            clear(mode).map_err(ExecuteError::TerminalClear)?;
            Ok(ExecutionResult::TerminalCleared)
        }
        Command::Exit => {
            let exit: Exit = exiter::exit;
            exit();
            Ok(ExecutionResult::Exit)
        }
        Command::Pipeline(_) => Err(ExecuteError::PipelineNotIntegrated),
    }
}
