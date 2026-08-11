use crate::definitions::contracts::create_dir;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub fn resolve(capability: create_dir::CreateDir, target: &str) -> Result<(), Error> {
    capability(target).map_err(|create_dir::Error::Unavailable| Error::Unavailable)
}
