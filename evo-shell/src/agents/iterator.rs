use crate::definitions::contracts::read_directory::ReadDirectory;
use crate::definitions::domain::entities::filesystem_iteration::FilesystemIteration;
use crate::definitions::domain::entities::filesystem_scope::FilesystemScope;
use crate::definitions::use_cases::iter::IterError;
use crate::resolvers::filesystem_iteration;

pub fn iter(
    scope: &FilesystemScope,
    read_directory: ReadDirectory,
) -> Result<FilesystemIteration, IterError> {
    filesystem_iteration::resolve(scope, read_directory)
}
