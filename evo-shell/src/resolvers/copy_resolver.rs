use crate::definitions::contracts::copy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub fn resolve(copy: copy::Copy, origin: &str, destination: &str) -> Result<(), Error> {
    copy(origin, destination).map_err(|copy::Error::Unavailable| Error::Unavailable)
}
