use std::io;

use crate::definitions::domain::entities::shell::Shell;
use crate::definitions::domain::value_objects::pipeline_value::PipelineValue;

pub type PresentPipelineResult =
    fn(shell: &Shell, value: PipelineValue) -> Result<(), PipelineResultPresentError>;

#[derive(Debug)]
pub enum PipelineResultPresentError {
    Io(io::Error),
}

impl From<io::Error> for PipelineResultPresentError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
