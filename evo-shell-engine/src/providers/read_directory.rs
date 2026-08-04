use std::fs;
use std::fs::ReadDir;
use std::io;
use std::path::Path;

pub fn provide(path: &Path) -> Result<ReadDir, io::Error> {
    fs::read_dir(path)
}
