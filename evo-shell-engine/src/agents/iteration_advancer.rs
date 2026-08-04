use crate::definitions::domain::entities::filesystem_entry::FilesystemEntry;
use crate::definitions::domain::entities::filesystem_iteration::FilesystemIteration;
use crate::definitions::use_cases::iter::IterError;
use crate::providers;
use crate::resolvers::filesystem_entry;

pub fn advance(iteration: &mut FilesystemIteration) -> Result<Option<FilesystemEntry>, IterError> {
    filesystem_entry::resolve(iteration, providers::next_directory_entry::provide)
}
