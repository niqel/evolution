use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct FilesystemScope {
    path: PathBuf,
}

impl FilesystemScope {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
