use std::io::{self, Write};

use crate::definitions::use_cases::pipeline_result_presenter::PipelineResultPresentError;

pub fn provide(rendered: &str) -> Result<(), PipelineResultPresentError> {
    let mut stdout = io::stdout();
    provide_to(&mut stdout, rendered)?;
    stdout.flush().map_err(PipelineResultPresentError::from)
}

pub(crate) fn provide_to(
    writer: &mut impl Write,
    rendered: &str,
) -> Result<(), PipelineResultPresentError> {
    writer.write_all(rendered.as_bytes())?;
    Ok(())
}
