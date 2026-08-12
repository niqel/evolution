use crate::support::terminal_capture;
use evo_shell::agents::welcomer;
use evo_shell::collaborators::shell_informant;
use evo_shell::definitions::use_cases::welcome;

#[test]
fn welcomer_success() {
    let (result, output) = terminal_capture::run_with_capture(welcomer::welcome);
    assert_eq!(result, Ok(()));

    let information = shell_informant::collaborate();
    let expected_header = "CatarinaSoft\n\nEvolution Shell\nVersion ";
    let expected_version = information.version;
    let expected_footer = "\nA lightweight functional shell.\n\nEvo shell is a life :)\n";

    assert!(output.starts_with(expected_header));
    assert!(output.contains(expected_version));
    assert!(output.ends_with(expected_footer));
}

#[test]
fn welcomer_translates_terminal_error() {
    let result = terminal_capture::run_with_fail(welcomer::welcome);
    assert_eq!(result, Err(welcome::Error::TerminalUnavailable));
}
