use crate::definitions::contracts::read_directory::ReadDirectory;
use crate::definitions::domain::entities::filesystem_iteration::FilesystemIteration;
use crate::definitions::domain::entities::filesystem_scope::FilesystemScope;
use crate::definitions::use_cases::iter::IterError;

pub fn resolve(
    scope: &FilesystemScope,
    read_directory: ReadDirectory,
) -> Result<FilesystemIteration, IterError> {
    read_directory(scope.path())
        .map(FilesystemIteration::new)
        .map_err(IterError::OpenDirectory)
}
