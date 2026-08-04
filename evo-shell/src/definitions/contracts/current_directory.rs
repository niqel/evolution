use std::io;
use std::path::PathBuf;

pub type CurrentDirectory = fn() -> Result<PathBuf, CurrentDirectoryError>;

pub type CurrentDirectoryError = io::Error;
