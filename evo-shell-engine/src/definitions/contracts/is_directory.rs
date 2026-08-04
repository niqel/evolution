use std::io;
use std::path::Path;

pub type FilesystemError = io::Error;

pub type IsDirectory = fn(path: &Path) -> Result<bool, FilesystemError>;
