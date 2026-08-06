use crate::definitions::domain::entities::command::Command;
use crate::definitions::domain::entities::shell::Shell;
use crate::definitions::use_cases::execute::{ExecuteError, ExecutionResult};
use crate::definitions::use_cases::execute_pipeline::ExecutePipeline;
use crate::definitions::use_cases::terminal_clearer::TerminalClearer;
use crate::resolvers::execution;
use crate::terminal_clearer;

pub fn execute(shell: &mut Shell, command: Command<'_>) -> Result<ExecutionResult, ExecuteError> {
    let clear: TerminalClearer = terminal_clearer::clear;
    let execute_pipeline: ExecutePipeline = crate::pipeline_executor::execute;

    execute_with(shell, command, clear, execute_pipeline)
}

pub(crate) fn execute_with(
    shell: &mut Shell,
    command: Command<'_>,
    clear: TerminalClearer,
    execute_pipeline: ExecutePipeline,
) -> Result<ExecutionResult, ExecuteError> {
    execution::resolve_with(shell, command, clear, execute_pipeline)
}
