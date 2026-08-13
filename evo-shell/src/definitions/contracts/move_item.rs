use crate::definitions::requesters::transfer_progress_requester;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub type Move = for<'origin, 'destination> fn(
    transfer_progress_requester::Request,
    &'origin str,
    &'destination str,
) -> Result<(), Error>;
