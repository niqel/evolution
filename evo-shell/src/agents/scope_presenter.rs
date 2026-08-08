use crate::callbacks::scope_consumer;
use crate::definitions::contracts::provide_scope;
use crate::definitions::contracts::write_terminal;
use crate::definitions::use_cases::present_scope;
use crate::resolvers::scope_resolver;

pub fn present(
    provide: provide_scope::Provide,
    write: write_terminal::Write,
) -> Result<(), present_scope::Error> {
    let consume = scope_consumer::consume;

    match scope_resolver::resolve(provide, consume, write) {
        Ok(()) => Ok(()),
        Err(scope_resolver::Error::Scope(_)) => Err(present_scope::Error::ScopeUnavailable),
        Err(scope_resolver::Error::Terminal(_)) => Err(present_scope::Error::TerminalUnavailable),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::callbacks::consume_scope;

    fn mock_write_success(_content: &str) -> Result<(), write_terminal::Error> {
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
    fn scope_presenter_success() {
        assert_eq!(present(mock_provide_success, mock_write_success), Ok(()));
    }

    #[test]
    fn scope_presenter_translates_scope_error() {
        assert_eq!(
            present(mock_provide_scope_error, mock_write_success),
            Err(present_scope::Error::ScopeUnavailable)
        );
    }

    #[test]
    fn scope_presenter_translates_terminal_error() {
        assert_eq!(
            present(mock_provide_terminal_error, mock_write_success),
            Err(present_scope::Error::TerminalUnavailable)
        );
    }
}
