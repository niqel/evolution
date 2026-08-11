use evo_shell::agents::file_creator;
use evo_shell::definitions::contracts::create_file;
use evo_shell::definitions::use_cases::create_file as create_file_use_case;
use std::sync::Mutex;

fn mock_create_file_success(_target: &str) -> Result<(), create_file::Error> {
    Ok(())
}

fn mock_create_file_unavailable(_target: &str) -> Result<(), create_file::Error> {
    Err(create_file::Error::Unavailable)
}

static CAPTURED_TARGET: Mutex<Option<String>> = Mutex::new(None);

fn mock_create_file_capture_target(target: &str) -> Result<(), create_file::Error> {
    let mut guard = CAPTURED_TARGET.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(target.to_string());
    Ok(())
}

#[test]
fn file_creator_success() {
    assert_eq!(
        file_creator::create_file(mock_create_file_success, "notes.txt"),
        Ok(())
    );
}

#[test]
fn file_creator_translates_error() {
    assert_eq!(
        file_creator::create_file(mock_create_file_unavailable, "notes.txt"),
        Err(create_file_use_case::Error::CreateFileUnavailable)
    );
}

#[test]
fn file_creator_transports_target() {
    {
        let mut guard = CAPTURED_TARGET.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    let result = file_creator::create_file(mock_create_file_capture_target, "documents/notes.txt");
    assert_eq!(result, Ok(()));

    let guard = CAPTURED_TARGET.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(guard.as_deref(), Some("documents/notes.txt"));
}
