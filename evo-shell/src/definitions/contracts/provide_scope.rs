use crate::definitions::callbacks::consume_scope;
use crate::definitions::contracts::write_terminal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub type Provide = fn(
    consume_scope::Consume,
    write_terminal::Write,
) -> Result<Result<(), consume_scope::Error>, Error>;
