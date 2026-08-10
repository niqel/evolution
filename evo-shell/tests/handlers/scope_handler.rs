use crate::support::terminal_capture;
use evo_shell::definitions::continuations::consume_scope;
use evo_shell::definitions::structs::borrowed::scope::Scope;
use evo_shell::handlers::scope_handler;

#[test]
fn scope_handler_writes_fragments_in_order() {
    let scope = Scope {
        scope_type: "fs",
        server: "test-server",
        user: "test-user",
        source: "/",
        item: Some("/home/user/downloads"),
    };
    let (result, output) =
        terminal_capture::run_with_capture(|write| scope_handler::handle(write, scope));
    assert_eq!(result, Ok(()));
    assert_eq!(output, "scope-fs …/downloads>");
}

#[test]
fn scope_handler_handles_root_location() {
    let scope = Scope {
        scope_type: "fs",
        server: "test-server",
        user: "test-user",
        source: "/",
        item: None,
    };
    let (result, output) =
        terminal_capture::run_with_capture(|write| scope_handler::handle(write, scope));
    assert_eq!(result, Ok(()));
    assert_eq!(output, "scope-fs …/>");
}

#[test]
fn scope_handler_translates_writer_error() {
    let scope = Scope {
        scope_type: "fs",
        server: "test-server",
        user: "test-user",
        source: "/",
        item: Some("/home/user/downloads"),
    };
    let result = terminal_capture::run_with_fail(|write| scope_handler::handle(write, scope));
    assert_eq!(result, Err(consume_scope::Error::TerminalUnavailable));
}
