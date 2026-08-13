use crate::definitions::contracts::move_item;
use crate::definitions::requesters::move_requester;
use crate::definitions::requesters::transfer_progress_requester;
use crate::definitions::use_cases::move_to;
use crate::resolvers::move_resolver;

pub fn move_to(
    origin: &str,
    destination: &str,
    progress: transfer_progress_requester::Request,
    request: move_requester::Request,
    move_operation: move_item::Move,
) {
    move_resolver::resolve(move_operation, origin, destination, progress, request);
}

pub const MOVE: move_to::Move = move_to;
