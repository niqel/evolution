use std::io;

pub type TerminalClearer = fn() -> Result<(), TerminalClearError>;

#[derive(Debug)]
pub enum TerminalClearError {
    Io(io::Error),
}

impl From<io::Error> for TerminalClearError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
