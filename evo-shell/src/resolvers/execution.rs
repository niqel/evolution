use std::path::Path;

use evo_shell_engine::{FilesystemScope, SetFilesystemScope, scope_setter};

use crate::definitions::domain::entities::command::Command;
use crate::definitions::use_cases::execute::ExecuteError;

pub fn resolve(command: Command<'_>) -> Result<FilesystemScope, ExecuteError> {
    match command {
        Command::ScopeFs(path) => {
            let set_scope: SetFilesystemScope = scope_setter::set;
            set_scope(Path::new(path)).map_err(ExecuteError::Engine)
        }
    }
}
