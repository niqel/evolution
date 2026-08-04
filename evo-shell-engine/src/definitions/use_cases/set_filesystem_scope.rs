use std::path::{Path, PathBuf};

use crate::definitions::contracts::is_directory::FilesystemError;
use crate::definitions::domain::entities::filesystem_scope::FilesystemScope;

pub type SetFilesystemScope = fn(path: &Path) -> Result<FilesystemScope, ScopeError>;

#[derive(Debug)]
pub enum ScopeError {
    Filesystem(FilesystemError),
    NotDirectory(PathBuf),
}
