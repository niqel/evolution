use crate::support::terminal_capture;
use evo_shell::agents::about_presenter;
use evo_shell::definitions::use_cases::present_about;

#[test]
fn about_presenter_success() {
    let (result, output) = terminal_capture::run_with_capture(about_presenter::present);
    assert_eq!(result, Ok(()));
    assert_eq!(
        output,
        "Evolution Shell\nVersion 0.1.0\nA lightweight functional shell.\n"
    );
}

#[test]
fn about_presenter_translates_terminal_error() {
    let result = terminal_capture::run_with_fail(about_presenter::present);
    assert_eq!(result, Err(present_about::Error::TerminalUnavailable));
}
