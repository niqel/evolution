use evo_shell_engine::ScopeError;

use crate::definitions::contracts::current_directory::CurrentDirectoryError;
use crate::definitions::domain::entities::shell::Shell;

pub type InitializeShell = fn() -> Result<Shell, InitializeShellError>;

#[derive(Debug)]
pub enum InitializeShellError {
    CurrentDirectory(CurrentDirectoryError),
    Scope(ScopeError),
}
