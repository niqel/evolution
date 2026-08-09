#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    ScopeUnavailable,
    TerminalUnavailable,
}

pub type Present = fn() -> Result<(), Error>;
