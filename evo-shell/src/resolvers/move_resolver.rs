use crate::definitions::contracts::move_item;
use crate::definitions::requesters::copy_progress_requester;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub fn resolve(
    capability: move_item::Move,
    report_progress: copy_progress_requester::Request,
    origin: &str,
    destination: &str,
) -> Result<(), Error> {
    capability(report_progress, origin, destination)
        .map_err(|move_item::Error::Unavailable| Error::Unavailable)
}
