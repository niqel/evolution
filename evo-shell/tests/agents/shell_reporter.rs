use crate::support::terminal_capture;
use evo_shell::agents::shell_reporter;
use evo_shell::definitions::use_cases::present_about;

#[test]
fn shell_reporter_success() {
    let (result, output) = terminal_capture::run_with_capture(shell_reporter::report);
    assert_eq!(result, Ok(()));
    assert_eq!(
        output,
        "Evolution Shell\nVersion 0.1.0\nA lightweight functional shell.\n"
    );
}

#[test]
fn shell_reporter_translates_terminal_error() {
    let result = terminal_capture::run_with_fail(shell_reporter::report);
    assert_eq!(result, Err(present_about::Error::TerminalUnavailable));
}
