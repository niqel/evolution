use evo_shell::agents::deleter;
use evo_shell::definitions::contracts::delete;
use evo_shell::definitions::use_cases::delete as delete_use_case;
use std::sync::Mutex;

fn mock_delete_success(_target: &str) -> Result<(), delete::Error> {
    Ok(())
}

fn mock_delete_unavailable(_target: &str) -> Result<(), delete::Error> {
    Err(delete::Error::Unavailable)
}

static CAPTURED_TARGET: Mutex<Option<String>> = Mutex::new(None);

fn mock_delete_capture_target(target: &str) -> Result<(), delete::Error> {
    let mut guard = CAPTURED_TARGET.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(target.to_string());
    Ok(())
}

#[test]
fn deleter_success() {
    assert_eq!(deleter::delete(mock_delete_success, "file.txt"), Ok(()));
}

#[test]
fn deleter_translates_delete_error() {
    assert_eq!(
        deleter::delete(mock_delete_unavailable, "file.txt"),
        Err(delete_use_case::Error::DeleteUnavailable)
    );
}

#[test]
fn deleter_transports_target() {
    {
        let mut guard = CAPTURED_TARGET.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    let result = deleter::delete(mock_delete_capture_target, "documents/file.txt");
    assert_eq!(result, Ok(()));

    let guard = CAPTURED_TARGET.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(guard.as_deref(), Some("documents/file.txt"));
}
