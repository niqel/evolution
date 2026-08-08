use crate::definitions::contracts::write_terminal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    TerminalUnavailable,
}

pub type Consume = for<'scope_type, 'location> fn(
    write_terminal::Write,
    &'scope_type str,
    &'location str,
) -> Result<(), Error>;
