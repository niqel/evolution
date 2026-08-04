use std::path::{Path, PathBuf};

use crate::definitions::domain::entities::filesystem_scope::FilesystemScope;

pub fn resolve(scope: &FilesystemScope, location: &Path) -> PathBuf {
    scope.path().join(location)
}
