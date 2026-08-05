use std::fs::ReadDir;

#[derive(Debug)]
pub struct FilesystemIteration {
    read_dir: ReadDir,
    next_index: usize,
}

impl FilesystemIteration {
    pub(crate) fn new(read_dir: ReadDir) -> Self {
        Self {
            read_dir,
            next_index: 0,
        }
    }

    pub(crate) fn read_dir_mut(&mut self) -> &mut ReadDir {
        &mut self.read_dir
    }

    pub(crate) fn next_index(&self) -> usize {
        self.next_index
    }

    pub(crate) fn advance_index(&mut self) {
        self.next_index += 1;
    }
}
