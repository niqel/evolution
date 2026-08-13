use crate::definitions::contracts::copy;
use crate::definitions::requesters::copy_requester;
use crate::definitions::requesters::transfer_progress_requester;
use crate::definitions::use_cases::copy_to;

pub fn resolve(
    copy: copy::Copy,
    origin: &str,
    destination: &str,
    progress: transfer_progress_requester::Request,
    request: copy_requester::Request,
) {
    let result = copy(progress, origin, destination).map_err(|_| copy_to::Error::CopyUnavailable);
    request(result);
}
