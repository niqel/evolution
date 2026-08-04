use evo_shell_engine::FilesystemScope;

use crate::definitions::domain::entities::command::Command;
use crate::definitions::use_cases::execute::ExecuteError;
use crate::resolvers::execution;

pub fn execute(command: Command<'_>) -> Result<FilesystemScope, ExecuteError> {
    execution::resolve(command)
}
