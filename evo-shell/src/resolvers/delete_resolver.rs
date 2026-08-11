use crate::definitions::contracts::delete;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub fn resolve(capability: delete::Delete, target: &str) -> Result<(), Error> {
    capability(target).map_err(|delete::Error::Unavailable| Error::Unavailable)
}
