use crate::definitions::domain::entities::filesystem_entry::FilesystemEntry;
use crate::definitions::domain::entities::filesystem_iteration::FilesystemIteration;
use crate::definitions::use_cases::iter::IterError;

pub type Advance =
    fn(iteration: &mut FilesystemIteration) -> Result<Option<FilesystemEntry>, IterError>;
