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

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_write(_content: &str) -> Result<(), write_terminal::Error> {
        Ok(())
    }

    fn mock_consume(
        _write: write_terminal::Write,
        _scope_type: &str,
        _location: &str,
    ) -> Result<(), consume_scope::Error> {
        Ok(())
    }

    fn mock_provide_success(
        consume: consume_scope::Consume,
        write: write_terminal::Write,
    ) -> Result<Result<(), consume_scope::Error>, provide_scope::Error> {
        Ok(consume(write, "fs", "/downloads"))
    }

    fn mock_provide_scope_error(
        _consume: consume_scope::Consume,
        _write: write_terminal::Write,
    ) -> Result<Result<(), consume_scope::Error>, provide_scope::Error> {
        Err(provide_scope::Error::Unavailable)
    }

    fn mock_provide_terminal_error(
        _consume: consume_scope::Consume,
        _write: write_terminal::Write,
    ) -> Result<Result<(), consume_scope::Error>, provide_scope::Error> {
        Ok(Err(consume_scope::Error::TerminalUnavailable))
    }

    #[test]
    fn scope_resolver_success() {
        assert_eq!(
            resolve(mock_provide_success, mock_consume, mock_write),
            Ok(())
        );
    }

    #[test]
    fn scope_resolver_distinguishes_scope_error() {
        assert_eq!(
            resolve(mock_provide_scope_error, mock_consume, mock_write),
            Err(Error::Scope(provide_scope::Error::Unavailable))
        );
    }

    #[test]
    fn scope_resolver_distinguishes_terminal_error() {
        assert_eq!(
            resolve(mock_provide_terminal_error, mock_consume, mock_write),
            Err(Error::Terminal(consume_scope::Error::TerminalUnavailable))
        );
    }
}
