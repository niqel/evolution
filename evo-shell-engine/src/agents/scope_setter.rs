use std::path::Path;

use crate::definitions::domain::entities::filesystem_scope::FilesystemScope;
use crate::definitions::use_cases::set_filesystem_scope::ScopeError;
use crate::providers;
use crate::resolvers::filesystem_scope;

pub fn set(path: &Path) -> Result<FilesystemScope, ScopeError> {
    filesystem_scope::resolve(path, providers::is_directory::provide)
}
