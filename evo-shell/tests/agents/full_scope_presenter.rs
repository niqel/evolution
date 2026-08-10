use evo_shell::agents::full_scope_presenter;
use evo_shell::definitions::continuations::consume_scope;
use evo_shell::definitions::contracts::{provide_scope, write_terminal};
use evo_shell::definitions::structs::borrowed::scope::Scope;
use evo_shell::definitions::use_cases::present_full_scope;

fn mock_write_success(_content: &str) -> Result<(), write_terminal::Error> {
    Ok(())
}

fn mock_provide_success(
    consume: consume_scope::Consume,
    write: write_terminal::Write,
) -> Result<Result<(), consume_scope::Error>, provide_scope::Error> {
    let scope = Scope {
        scope_type: "fs",
        server: "niqel-pc",
        user: "niqel504",
        source: "/",
        item: Some("/home/niqel504/repos/evolution"),
    };
    Ok(consume(write, scope))
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
fn full_scope_presenter_success() {
    assert_eq!(
        full_scope_presenter::present(mock_provide_success, mock_write_success),
        Ok(())
    );
}

#[test]
fn full_scope_presenter_translates_scope_error() {
    assert_eq!(
        full_scope_presenter::present(mock_provide_scope_error, mock_write_success),
        Err(present_full_scope::Error::ScopeUnavailable)
    );
}

#[test]
fn full_scope_presenter_translates_terminal_error() {
    assert_eq!(
        full_scope_presenter::present(mock_provide_terminal_error, mock_write_success),
        Err(present_full_scope::Error::TerminalUnavailable)
    );
}
