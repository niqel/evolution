use crate::definitions::domain::entities::filesystem_iteration::FilesystemIteration;
use crate::definitions::domain::entities::filesystem_iteration_item::FilesystemIterationItem;
use crate::definitions::use_cases::iter::IterError;

pub type Advance =
    fn(iteration: &mut FilesystemIteration) -> Result<Option<FilesystemIterationItem>, IterError>;
