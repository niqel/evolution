use crate::definitions::domain::entities::filesystem_entry::FilesystemEntry;

#[derive(Debug)]
pub struct FilesystemIterationItem {
    index: usize,
    entry: FilesystemEntry,
}

impl FilesystemIterationItem {
    pub(crate) fn new(index: usize, entry: FilesystemEntry) -> Self {
        Self { index, entry }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn entry(&self) -> &FilesystemEntry {
        &self.entry
    }
}
