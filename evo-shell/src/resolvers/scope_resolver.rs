use crate::definitions::callbacks::consume_scope;
use crate::definitions::contracts::provide_scope;
use crate::definitions::contracts::write_terminal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Scope(provide_scope::Error),
    Terminal(consume_scope::Error),
}

pub fn resolve(
    provide: provide_scope::Provide,
    consume: consume_scope::Consume,
    write: write_terminal::Write,
) -> Result<(), Error> {
    match provide(consume, write) {
        Ok(Ok(())) => Ok(()),
        Ok(Err(terminal_err)) => Err(Error::Terminal(terminal_err)),
        Err(scope_err) => Err(Error::Scope(scope_err)),
    }
}
