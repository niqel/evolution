use crate::definitions::continuations::report_copy_progress;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub type Copy = for<'origin, 'destination> fn(
    report_progress: report_copy_progress::Report,
    &'origin str,
    &'destination str,
) -> Result<(), Error>;
