use std::path::Path;

use evo_shell_engine::{Enter, Iter, SetFilesystemScope, enterer, iterator, scope_setter};

use crate::definitions::domain::entities::command::Command;
use crate::definitions::domain::entities::shell::Shell;
use crate::definitions::use_cases::execute::{ExecuteError, ExecutionResult};

pub fn resolve(shell: &mut Shell, command: Command<'_>) -> Result<ExecutionResult, ExecuteError> {
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
    }
}
