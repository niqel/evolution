use crate::definitions::domain::entities::command::Command;
use crate::definitions::domain::entities::shell::Shell;
use crate::definitions::use_cases::execute::{ExecuteError, ExecutionResult};
use crate::resolvers::execution;

pub fn execute(shell: &mut Shell, command: Command<'_>) -> Result<ExecutionResult, ExecuteError> {
    execution::resolve(shell, command)
}
