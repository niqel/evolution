use crate::definitions::contracts::write_terminal;
use crate::definitions::structs::borrowed::scope::Scope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    TerminalUnavailable,
}

pub type Consume = for<'scope> fn(write_terminal::Write, Scope<'scope>) -> Result<(), Error>;
