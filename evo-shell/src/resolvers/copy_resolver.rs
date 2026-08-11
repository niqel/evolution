use crate::definitions::continuations::report_copy_progress;
use crate::definitions::contracts::copy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub fn resolve(
    capability: copy::Copy,
    report_progress: report_copy_progress::Report,
    origin: &str,
    destination: &str,
) -> Result<(), Error> {
    capability(report_progress, origin, destination)
        .map_err(|copy::Error::Unavailable| Error::Unavailable)
}
