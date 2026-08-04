use crate::definitions::domain::entities::filesystem_iteration::FilesystemIteration;
use crate::definitions::domain::entities::filesystem_scope::FilesystemScope;
use std::io;

pub type Iter = fn(scope: &FilesystemScope) -> Result<FilesystemIteration, IterError>;

#[derive(Debug)]
pub enum IterError {
    OpenDirectory(io::Error),
    NextEntry(io::Error),
    MaterializeEntry(io::Error),
}
