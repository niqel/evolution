use crate::definitions::contracts::move_item;
use crate::definitions::requesters::copy_progress_requester;
use crate::definitions::use_cases::move_to;
use crate::resolvers::move_resolver;

pub fn move_to(
    capability: move_item::Move,
    report_progress: copy_progress_requester::Request,
    origin: &str,
    destination: &str,
) -> Result<(), move_to::Error> {
    move_resolver::resolve(capability, report_progress, origin, destination)
        .map_err(|_| move_to::Error::MoveUnavailable)
}
