use crate::definitions::requesters::copy_progress_requester;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub type Copy = for<'origin, 'destination> fn(
    copy_progress_requester::Request,
    &'origin str,
    &'destination str,
) -> Result<(), Error>;
