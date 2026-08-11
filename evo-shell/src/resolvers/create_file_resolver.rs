use crate::definitions::contracts::create_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub fn resolve(capability: create_file::CreateFile, target: &str) -> Result<(), Error> {
    capability(target).map_err(|create_file::Error::Unavailable| Error::Unavailable)
}
