use std::io;

use crate::definitions::domain::value_objects::terminal_clear_mode::TerminalClearMode;

pub type TerminalClearer = fn(TerminalClearMode) -> Result<(), TerminalClearError>;

#[derive(Debug)]
pub enum TerminalClearError {
    Io(io::Error),
}

impl From<io::Error> for TerminalClearError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}
