use std::fs;
use std::path::Path;

use crate::definitions::contracts::is_directory::FilesystemError;

pub fn provide(path: &Path) -> Result<bool, FilesystemError> {
    Ok(fs::metadata(path)?.is_dir())
}
