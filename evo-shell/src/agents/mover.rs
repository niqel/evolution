use crate::definitions::continuations::report_copy_progress;
use crate::definitions::contracts::move_item;
use crate::definitions::use_cases::move_to;
use crate::resolvers::move_resolver;

pub fn move_to(
    capability: move_item::Move,
    report_progress: report_copy_progress::Report,
    origin: &str,
    destination: &str,
) -> Result<(), move_to::Error> {
    match move_resolver::resolve(capability, report_progress, origin, destination) {
        Ok(()) => Ok(()),
        Err(move_resolver::Error::Unavailable) => Err(move_to::Error::MoveUnavailable),
    }
}
