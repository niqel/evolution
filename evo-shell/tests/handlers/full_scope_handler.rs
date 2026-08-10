use crate::support::terminal_capture;
use evo_shell::definitions::continuations::consume_scope;
use evo_shell::definitions::structs::borrowed::scope::Scope;
use evo_shell::handlers::full_scope_handler;

#[test]
fn full_scope_handler_db_with_item() {
    let scope = Scope {
        scope_type: "db",
        server: "sql-server",
        user: "gustavo",
        source: "miBaseDatos",
        item: Some("trabajadores"),
    };
    let (result, output) =
        terminal_capture::run_with_capture(|write| full_scope_handler::handle(write, scope));
    assert_eq!(result, Ok(()));
    assert_eq!(output, "sql-server/gustavo/miBaseDatos/trabajadores\n");
}

#[test]
fn full_scope_handler_db_without_item() {
    let scope = Scope {
        scope_type: "db",
        server: "sql-server",
        user: "gustavo",
        source: "miBaseDatos",
        item: None,
    };
    let (result, output) =
        terminal_capture::run_with_capture(|write| full_scope_handler::handle(write, scope));
    assert_eq!(result, Ok(()));
    assert_eq!(output, "sql-server/gustavo/miBaseDatos\n");
}

#[test]
fn full_scope_handler_fs_root_source_with_item() {
    let scope = Scope {
        scope_type: "fs",
        server: "niqel-pc",
        user: "niqel504",
        source: "/",
        item: Some("/home/niqel504/repos/evolution"),
    };
    let (result, output) =
        terminal_capture::run_with_capture(|write| full_scope_handler::handle(write, scope));
    assert_eq!(result, Ok(()));
    assert_eq!(output, "niqel-pc/niqel504/home/niqel504/repos/evolution\n");
}

#[test]
fn full_scope_handler_translates_writer_error() {
    let scope = Scope {
        scope_type: "fs",
        server: "niqel-pc",
        user: "niqel504",
        source: "/",
        item: Some("/home/niqel504/repos/evolution"),
    };
    let result = terminal_capture::run_with_fail(|write| full_scope_handler::handle(write, scope));
    assert_eq!(result, Err(consume_scope::Error::TerminalUnavailable));
}
