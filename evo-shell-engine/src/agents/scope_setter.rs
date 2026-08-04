use std::path::Path;

use crate::definitions::contracts::is_directory::IsDirectory;
use crate::definitions::domain::entities::filesystem_scope::FilesystemScope;
use crate::definitions::use_cases::set_filesystem_scope::ScopeError;
use crate::resolvers::filesystem_scope;

pub fn set(path: &Path, is_directory: IsDirectory) -> Result<FilesystemScope, ScopeError> {
    filesystem_scope::resolve(path, is_directory)
}
