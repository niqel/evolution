use std::fs::ReadDir;
use std::io;
use std::path::Path;

pub type ReadDirectory = fn(path: &Path) -> Result<ReadDir, io::Error>;
