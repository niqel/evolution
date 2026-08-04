use std::fs::DirEntry;
use std::io;

use crate::definitions::domain::entities::filesystem_iteration::FilesystemIteration;

pub fn provide(iteration: &mut FilesystemIteration) -> Result<Option<DirEntry>, io::Error> {
    iteration.read_dir_mut().next().transpose()
}
