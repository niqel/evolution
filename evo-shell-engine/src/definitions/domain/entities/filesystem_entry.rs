use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

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
    modified: Option<SystemTime>,
    size: Option<u64>,
}

impl FilesystemEntry {
    pub(crate) fn new(
        name: OsString,
        path: PathBuf,
        kind: FilesystemEntryKind,
        modified: Option<SystemTime>,
        size: Option<u64>,
    ) -> Self {
        Self {
            name,
            path,
            kind,
            modified,
            size,
        }
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

    pub fn modified(&self) -> Option<SystemTime> {
        self.modified
    }

    pub fn size(&self) -> Option<u64> {
        self.size
    }
}
