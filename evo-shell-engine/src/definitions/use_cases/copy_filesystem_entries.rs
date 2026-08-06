use std::fmt;
use std::path::{Path, PathBuf};

use crate::definitions::domain::entities::filesystem_scope::FilesystemScope;

#[derive(Debug)]
pub enum CopyError {
    SourceNotFound(PathBuf),
    DestinationNotFound(PathBuf),
    DestinationNotDirectory(PathBuf),
    DestinationAlreadyExists(PathBuf),
    RecursiveSelfCopy(PathBuf),
    UnsupportedSourceType(PathBuf),
    Filesystem(std::io::Error),
}

impl fmt::Display for CopyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceNotFound(path) => write!(f, "source path not found: {}", path.display()),
            Self::DestinationNotFound(path) => {
                write!(f, "destination path not found: {}", path.display())
            }
            Self::DestinationNotDirectory(path) => {
                write!(f, "destination is not a directory: {}", path.display())
            }
            Self::DestinationAlreadyExists(path) => {
                write!(f, "destination target already exists: {}", path.display())
            }
            Self::RecursiveSelfCopy(path) => {
                write!(
                    f,
                    "cannot copy directory into itself or onto itself: {}",
                    path.display()
                )
            }
            Self::UnsupportedSourceType(path) => {
                write!(
                    f,
                    "unsupported source entry type (symlink): {}",
                    path.display()
                )
            }
            Self::Filesystem(err) => write!(f, "filesystem error: {err}"),
        }
    }
}

impl std::error::Error for CopyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Filesystem(err) => Some(err),
            _ => None,
        }
    }
}

pub type CopyFilesystemEntries =
    fn(scope: &FilesystemScope, sources: &[&Path], destination: &Path) -> Result<(), CopyError>;
