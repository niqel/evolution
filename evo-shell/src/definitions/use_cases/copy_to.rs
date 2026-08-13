use crate::definitions::contracts::copy;
use crate::definitions::requesters::copy_progress_requester;
use crate::definitions::requesters::copy_requester;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    CopyUnavailable,
}

pub type Copy = for<'origin, 'destination> fn(
    &'origin str,
    &'destination str,
    copy_progress_requester::Request,
    copy_requester::Request,
    copy::Copy,
);
