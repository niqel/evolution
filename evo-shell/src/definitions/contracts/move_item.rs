use crate::definitions::requesters::copy_progress_requester;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub type Move = for<'origin, 'destination> fn(
    report_progress: copy_progress_requester::Request,
    &'origin str,
    &'destination str,
) -> Result<(), Error>;
