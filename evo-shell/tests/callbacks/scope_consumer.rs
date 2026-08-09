use crate::support::terminal_capture;
use evo_shell::callbacks::scope_consumer;
use evo_shell::definitions::callbacks::consume_scope;

#[test]
fn scope_consumer_writes_fragments_in_order() {
    let (result, output) = terminal_capture::run_with_capture(|write| {
        scope_consumer::consume(write, "fs", "/home/user/downloads")
    });
    assert_eq!(result, Ok(()));
    assert_eq!(output, "scope-fs …/downloads>");
}

#[test]
fn scope_consumer_handles_root_location() {
    let (result, output) =
        terminal_capture::run_with_capture(|write| scope_consumer::consume(write, "fs", "/"));
    assert_eq!(result, Ok(()));
    assert_eq!(output, "scope-fs …/>");
}

#[test]
fn scope_consumer_translates_writer_error() {
    let result = terminal_capture::run_with_fail(|write| {
        scope_consumer::consume(write, "fs", "/home/user/downloads")
    });
    assert_eq!(result, Err(consume_scope::Error::TerminalUnavailable));
}
