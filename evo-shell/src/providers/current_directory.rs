use std::path::PathBuf;

use crate::definitions::contracts::current_directory::CurrentDirectoryError;

pub fn provide() -> Result<PathBuf, CurrentDirectoryError> {
    std::env::current_dir()
}
