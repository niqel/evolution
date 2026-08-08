#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    TerminalUnavailable,
}

pub type Present = fn() -> Result<(), Error>;
