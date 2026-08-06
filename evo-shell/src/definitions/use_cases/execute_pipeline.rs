use evo_shell_engine::{
    FilterError, IndexError, SelectError, ToArgsError, ToValueError, ToValuesError,
};

use crate::definitions::domain::entities::shell::Shell;
use crate::definitions::domain::value_objects::pipeline::{Pipeline, PipelineOperationKind};
use crate::definitions::domain::value_objects::pipeline_value::{PipelineValue, PipelineValueKind};

pub type ExecutePipeline =
    fn(shell: &Shell, pipeline: Pipeline) -> Result<PipelineValue, PipelineExecutionError>;

#[derive(Debug)]
pub enum PipelineExecutionError {
    EmptyPipeline,
    InvalidInitialOperation {
        operation: PipelineOperationKind,
    },
    InvalidTransition {
        operation: PipelineOperationKind,
        state: PipelineValueKind,
    },
    Iter(evo_shell_engine::IterError),
    Filter(FilterError),
    Index(IndexError),
    Select(SelectError),
    ToValue(ToValueError),
    ToValues(ToValuesError),
    ToArgs(ToArgsError),
}

impl From<evo_shell_engine::IterError> for PipelineExecutionError {
    fn from(error: evo_shell_engine::IterError) -> Self {
        Self::Iter(error)
    }
}

impl From<FilterError> for PipelineExecutionError {
    fn from(error: FilterError) -> Self {
        Self::Filter(error)
    }
}

impl From<IndexError> for PipelineExecutionError {
    fn from(error: IndexError) -> Self {
        Self::Index(error)
    }
}

impl From<SelectError> for PipelineExecutionError {
    fn from(error: SelectError) -> Self {
        Self::Select(error)
    }
}

impl From<ToValueError> for PipelineExecutionError {
    fn from(error: ToValueError) -> Self {
        Self::ToValue(error)
    }
}

impl From<ToValuesError> for PipelineExecutionError {
    fn from(error: ToValuesError) -> Self {
        Self::ToValues(error)
    }
}

impl From<ToArgsError> for PipelineExecutionError {
    fn from(error: ToArgsError) -> Self {
        Self::ToArgs(error)
    }
}
