use evo_shell::definitions::contracts::create_file;
use evo_shell::definitions::use_cases::create_file as create_file_use_case;
use evo_shell::resolvers::create_file_resolver;
use std::sync::Mutex;

fn mock_create_file_success(_target: &str) -> Result<(), create_file::Error> {
    Ok(())
}

fn mock_create_file_unavailable(_target: &str) -> Result<(), create_file::Error> {
    Err(create_file::Error::Unavailable)
}

static CAPTURED_RESULT: Mutex<Option<Result<(), create_file_use_case::Error>>> = Mutex::new(None);

fn mock_final_request(result: Result<(), create_file_use_case::Error>) {
    let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(result);
}

#[test]
fn create_file_resolver_success() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    create_file_resolver::resolve(mock_create_file_success, "notes.txt", mock_final_request);

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Ok(())));
}

#[test]
fn create_file_resolver_translates_error() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    create_file_resolver::resolve(
        mock_create_file_unavailable,
        "notes.txt",
        mock_final_request,
    );

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(
        *guard,
        Some(Err(create_file_use_case::Error::CreateFileUnavailable))
    );
}
