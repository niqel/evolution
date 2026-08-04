use std::path::Path;

use crate::definitions::domain::entities::filesystem_scope::FilesystemScope;
use crate::definitions::use_cases::set_filesystem_scope::ScopeError;

pub type Enter =
    fn(scope: &FilesystemScope, location: &Path) -> Result<FilesystemScope, ScopeError>;
