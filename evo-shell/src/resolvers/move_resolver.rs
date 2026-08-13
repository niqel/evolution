use crate::definitions::contracts::move_item;
use crate::definitions::requesters::move_requester;
use crate::definitions::requesters::transfer_progress_requester;
use crate::definitions::use_cases::move_to;

pub fn resolve(
    move_operation: move_item::Move,
    origin: &str,
    destination: &str,
    progress: transfer_progress_requester::Request,
    request: move_requester::Request,
) {
    let result =
        move_operation(progress, origin, destination).map_err(|_| move_to::Error::MoveUnavailable);
    request(result);
}
