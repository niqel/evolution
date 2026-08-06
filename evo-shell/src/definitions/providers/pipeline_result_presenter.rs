use crate::definitions::use_cases::pipeline_result_presenter::PipelineResultPresentError;

pub type Provide = fn(&str) -> Result<(), PipelineResultPresentError>;
