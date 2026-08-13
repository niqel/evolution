use crate::definitions::contracts::copy;
use crate::definitions::requesters::copy_requester;
use crate::definitions::requesters::transfer_progress_requester;
use crate::definitions::use_cases::copy_to;
use crate::resolvers::copy_resolver;

pub fn copy(
    origin: &str,
    destination: &str,
    progress: transfer_progress_requester::Request,
    request: copy_requester::Request,
    copy: copy::Copy,
) {
    copy_resolver::resolve(copy, origin, destination, progress, request);
}

pub const COPY: copy_to::Copy = copy;
