use crate::definitions::contracts::copy;
use crate::definitions::requesters::copy_requester;
use crate::definitions::requesters::transfer_progress_requester;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    CopyUnavailable,
}

pub type Copy = for<'origin, 'destination> fn(
    &'origin str,
    &'destination str,
    transfer_progress_requester::Request,
    copy_requester::Request,
    copy::Copy,
);
