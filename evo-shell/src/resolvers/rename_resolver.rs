use crate::definitions::contracts::rename;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub fn resolve(capability: rename::Rename, target: &str, new_name: &str) -> Result<(), Error> {
    capability(target, new_name).map_err(|rename::Error::Unavailable| Error::Unavailable)
}
