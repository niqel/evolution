use std::path::Path;

use crate::definitions::contracts::is_directory::IsDirectory;
use crate::definitions::domain::entities::filesystem_scope::FilesystemScope;
use crate::definitions::use_cases::set_filesystem_scope::ScopeError;

pub fn resolve(path: &Path, is_directory: IsDirectory) -> Result<FilesystemScope, ScopeError> {
    if is_directory(path).map_err(ScopeError::Filesystem)? {
        Ok(FilesystemScope::new(path.to_path_buf()))
    } else {
        Err(ScopeError::NotDirectory(path.to_path_buf()))
    }
}
