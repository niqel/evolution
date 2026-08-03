use std::fs::DirEntry;
use std::io;

use crate::definitions::domain::entities::filesystem_iteration::FilesystemIteration;

pub type NextDirectoryEntry =
    fn(iteration: &mut FilesystemIteration) -> Result<Option<DirEntry>, io::Error>;
