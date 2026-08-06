use evo_shell_engine::{FilesystemIteration, IterError, ScopeError};

use crate::definitions::domain::entities::command::Command;
use crate::definitions::domain::entities::shell::Shell;
use crate::definitions::domain::value_objects::pipeline_value::PipelineValue;
use crate::definitions::use_cases::execute_pipeline::PipelineExecutionError;
use crate::definitions::use_cases::terminal_clearer::TerminalClearError;

pub type Execute =
    for<'a> fn(shell: &mut Shell, command: Command<'a>) -> Result<ExecutionResult, ExecuteError>;

#[derive(Debug)]
pub enum ExecutionResult {
    ScopeChanged,
    FilesystemIteration(FilesystemIteration),
    TerminalCleared,
    Exit,
    Pipeline(PipelineValue),
}

#[derive(Debug)]
pub enum ExecuteError {
    Scope(ScopeError),
    Iter(IterError),
    TerminalClear(TerminalClearError),
    Pipeline(PipelineExecutionError),
    IncompatibleGroupedArgument,
}

impl From<PipelineExecutionError> for ExecuteError {
    fn from(error: PipelineExecutionError) -> Self {
        Self::Pipeline(error)
    }
}
