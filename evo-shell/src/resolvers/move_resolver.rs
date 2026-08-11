use crate::definitions::continuations::report_copy_progress;
use crate::definitions::contracts::move_item;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub fn resolve(
    capability: move_item::Move,
    report_progress: report_copy_progress::Report,
    origin: &str,
    destination: &str,
) -> Result<(), Error> {
    capability(report_progress, origin, destination)
        .map_err(|move_item::Error::Unavailable| Error::Unavailable)
}
