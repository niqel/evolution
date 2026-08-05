use std::path::Path;

use crate::definitions::contracts::is_directory::IsDirectory;
use crate::definitions::domain::entities::filesystem_scope::FilesystemScope;
use crate::definitions::use_cases::set_filesystem_scope::ScopeError;

pub fn resolve(path: &Path, is_directory: IsDirectory) -> Result<FilesystemScope, ScopeError> {
    let resolved_path = path.canonicalize().map_err(ScopeError::Filesystem)?;

    if is_directory(&resolved_path).map_err(ScopeError::Filesystem)? {
        Ok(FilesystemScope::new(resolved_path))
    } else {
        Err(ScopeError::NotDirectory(resolved_path))
    }
}
