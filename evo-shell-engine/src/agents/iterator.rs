use crate::definitions::domain::entities::filesystem_iteration::FilesystemIteration;
use crate::definitions::domain::entities::filesystem_scope::FilesystemScope;
use crate::definitions::use_cases::iter::IterError;
use crate::providers;
use crate::resolvers::filesystem_iteration;

pub fn iter(scope: &FilesystemScope) -> Result<FilesystemIteration, IterError> {
    filesystem_iteration::resolve(scope, providers::read_directory::provide)
}
