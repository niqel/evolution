use evo_shell::agents::trasher;
use evo_shell::definitions::contracts::trash;
use evo_shell::definitions::use_cases::trash as trash_use_case;
use std::sync::Mutex;

fn mock_trash_success(_target: &str) -> Result<(), trash::Error> {
    Ok(())
}

fn mock_trash_unavailable(_target: &str) -> Result<(), trash::Error> {
    Err(trash::Error::Unavailable)
}

static CAPTURED_TARGET: Mutex<Option<String>> = Mutex::new(None);

fn mock_trash_capture_target(target: &str) -> Result<(), trash::Error> {
    let mut guard = CAPTURED_TARGET.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(target.to_string());
    Ok(())
}

#[test]
fn trasher_success() {
    assert_eq!(trasher::trash(mock_trash_success, "file.txt"), Ok(()));
}

#[test]
fn trasher_translates_trash_error() {
    assert_eq!(
        trasher::trash(mock_trash_unavailable, "file.txt"),
        Err(trash_use_case::Error::TrashUnavailable)
    );
}

#[test]
fn trasher_transports_target() {
    {
        let mut guard = CAPTURED_TARGET.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    let result = trasher::trash(mock_trash_capture_target, "documents/file.txt");
    assert_eq!(result, Ok(()));

    let guard = CAPTURED_TARGET.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(guard.as_deref(), Some("documents/file.txt"));
}
