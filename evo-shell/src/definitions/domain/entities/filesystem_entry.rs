use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemEntryKind {
    File,
    Directory,
    Symlink,
    Other,
}

#[derive(Debug)]
pub struct FilesystemEntry {
    name: OsString,
    path: PathBuf,
    kind: FilesystemEntryKind,
}

impl FilesystemEntry {
    pub(crate) fn new(name: OsString, path: PathBuf, kind: FilesystemEntryKind) -> Self {
        Self { name, path, kind }
    }

    pub fn name(&self) -> &OsStr {
        &self.name
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn kind(&self) -> FilesystemEntryKind {
        self.kind
    }
}
