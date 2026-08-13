use evo_shell::definitions::contracts::rename;
use evo_shell::definitions::use_cases::rename as rename_use_case;
use evo_shell::resolvers::rename_resolver;
use std::sync::Mutex;

fn mock_rename_success(_target: &str, _new_name: &str) -> Result<(), rename::Error> {
    Ok(())
}

fn mock_rename_unavailable(_target: &str, _new_name: &str) -> Result<(), rename::Error> {
    Err(rename::Error::Unavailable)
}

static CAPTURED_RESULT: Mutex<Option<Result<(), rename_use_case::Error>>> = Mutex::new(None);

fn mock_final_request(result: Result<(), rename_use_case::Error>) {
    let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(result);
}

#[test]
fn rename_resolver_success() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    rename_resolver::resolve(
        mock_rename_success,
        "videos/gatito.mp4",
        "michi.mp4",
        mock_final_request,
    );

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Ok(())));
}

#[test]
fn rename_resolver_translates_error() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    rename_resolver::resolve(
        mock_rename_unavailable,
        "videos/gatito.mp4",
        "michi.mp4",
        mock_final_request,
    );

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Err(rename_use_case::Error::RenameUnavailable)));
}
