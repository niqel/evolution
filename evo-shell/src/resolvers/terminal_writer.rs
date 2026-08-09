use crate::definitions::contracts::write_terminal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub fn resolve(write: write_terminal::Write, content: &str) -> Result<(), Error> {
    write(content).map_err(|write_terminal::Error::Unavailable| Error::Unavailable)
}
