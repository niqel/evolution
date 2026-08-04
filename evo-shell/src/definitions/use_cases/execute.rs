use evo_shell_engine::{FilesystemScope, ScopeError};

use crate::definitions::domain::entities::command::Command;

pub type Execute = for<'a> fn(command: Command<'a>) -> Result<FilesystemScope, ExecuteError>;

#[derive(Debug)]
pub enum ExecuteError {
    Engine(ScopeError),
}
