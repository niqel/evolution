use crate::definitions::continuations::report_copy_progress;
use crate::definitions::contracts::copy;
use crate::definitions::use_cases::copy_to;
use crate::resolvers::copy_resolver;

pub fn copy(
    capability: copy::Copy,
    report_progress: report_copy_progress::Report,
    origin: &str,
    destination: &str,
) -> Result<(), copy_to::Error> {
    copy_resolver::resolve(capability, report_progress, origin, destination)
        .map_err(|_| copy_to::Error::CopyUnavailable)
}
