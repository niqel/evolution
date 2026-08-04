use std::path::Path;

use crate::agents::scope_setter;
use crate::definitions::domain::entities::filesystem_scope::FilesystemScope;
use crate::definitions::use_cases::set_filesystem_scope::ScopeError;
use crate::resolvers::filesystem_path;

pub fn enter(scope: &FilesystemScope, location: &Path) -> Result<FilesystemScope, ScopeError> {
    let path = filesystem_path::resolve(scope, location);
    scope_setter::set(&path)
}
