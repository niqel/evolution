#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    TerminalUnavailable,
}

pub type Welcome = fn() -> Result<(), Error>;
