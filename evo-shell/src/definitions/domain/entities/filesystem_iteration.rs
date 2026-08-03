use std::fs::ReadDir;

#[derive(Debug)]
pub struct FilesystemIteration {
    read_dir: ReadDir,
}

impl FilesystemIteration {
    pub(crate) fn new(read_dir: ReadDir) -> Self {
        Self { read_dir }
    }

    pub(crate) fn read_dir_mut(&mut self) -> &mut ReadDir {
        &mut self.read_dir
    }
}
