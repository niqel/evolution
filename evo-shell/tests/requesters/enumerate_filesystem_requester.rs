use evo_shell::definitions::requesters::enumerate_filesystem_requester;
use evo_shell::definitions::use_cases::enumerate_filesystem;

fn receive_success(result: Result<(), enumerate_filesystem::Error>) {
    assert_eq!(result, Ok(()));
}

fn receive_error(result: Result<(), enumerate_filesystem::Error>) {
    assert_eq!(
        result,
        Err(enumerate_filesystem::Error::EnumerationUnavailable)
    );
}

#[test]
fn enumerate_filesystem_requester_handles_success() {
    let request: enumerate_filesystem_requester::Request = receive_success;
    request(Ok(()));
}

#[test]
fn enumerate_filesystem_requester_handles_error() {
    let request: enumerate_filesystem_requester::Request = receive_error;
    request(Err(enumerate_filesystem::Error::EnumerationUnavailable));
}
