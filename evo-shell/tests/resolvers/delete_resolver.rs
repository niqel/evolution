use evo_shell::definitions::contracts::delete;
use evo_shell::definitions::use_cases::delete as delete_use_case;
use evo_shell::resolvers::delete_resolver;
use std::sync::Mutex;

fn mock_delete_success(_target: &str) -> Result<(), delete::Error> {
    Ok(())
}

fn mock_delete_unavailable(_target: &str) -> Result<(), delete::Error> {
    Err(delete::Error::Unavailable)
}

static CAPTURED_RESULT: Mutex<Option<Result<(), delete_use_case::Error>>> = Mutex::new(None);

fn mock_final_request(result: Result<(), delete_use_case::Error>) {
    let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(result);
}

#[test]
fn delete_resolver_success() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    delete_resolver::resolve(mock_delete_success, "file.txt", mock_final_request);

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Ok(())));
}

#[test]
fn delete_resolver_translates_error() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    delete_resolver::resolve(mock_delete_unavailable, "file.txt", mock_final_request);

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Err(delete_use_case::Error::DeleteUnavailable)));
}
