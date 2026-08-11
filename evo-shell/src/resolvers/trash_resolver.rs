use crate::definitions::contracts::trash;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub fn resolve(capability: trash::Trash, target: &str) -> Result<(), Error> {
    capability(target).map_err(|trash::Error::Unavailable| Error::Unavailable)
}
