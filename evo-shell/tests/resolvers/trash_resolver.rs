use evo_shell::definitions::contracts::trash;
use evo_shell::definitions::use_cases::trash as trash_use_case;
use evo_shell::resolvers::trash_resolver;
use std::sync::Mutex;

fn mock_trash_success(_target: &str) -> Result<(), trash::Error> {
    Ok(())
}

fn mock_trash_unavailable(_target: &str) -> Result<(), trash::Error> {
    Err(trash::Error::Unavailable)
}

static CAPTURED_RESULT: Mutex<Option<Result<(), trash_use_case::Error>>> = Mutex::new(None);

fn mock_final_request(result: Result<(), trash_use_case::Error>) {
    let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(result);
}

#[test]
fn trash_resolver_success() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    trash_resolver::resolve(mock_trash_success, "file.txt", mock_final_request);

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Ok(())));
}

#[test]
fn trash_resolver_translates_error() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    trash_resolver::resolve(mock_trash_unavailable, "file.txt", mock_final_request);

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Err(trash_use_case::Error::TrashUnavailable)));
}
