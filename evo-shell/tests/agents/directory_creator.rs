use evo_shell::agents::directory_creator;
use evo_shell::definitions::contracts::create_dir;
use evo_shell::definitions::use_cases::create_dir as create_dir_use_case;
use std::sync::Mutex;

fn mock_create_dir_success(_target: &str) -> Result<(), create_dir::Error> {
    Ok(())
}

fn mock_create_dir_unavailable(_target: &str) -> Result<(), create_dir::Error> {
    Err(create_dir::Error::Unavailable)
}

static CAPTURED_TARGET: Mutex<Option<String>> = Mutex::new(None);

fn mock_create_dir_capture_target(target: &str) -> Result<(), create_dir::Error> {
    let mut guard = CAPTURED_TARGET.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(target.to_string());
    Ok(())
}

#[test]
fn directory_creator_success() {
    assert_eq!(
        directory_creator::create_dir(mock_create_dir_success, "documents"),
        Ok(())
    );
}

#[test]
fn directory_creator_translates_error() {
    assert_eq!(
        directory_creator::create_dir(mock_create_dir_unavailable, "documents"),
        Err(create_dir_use_case::Error::CreateDirUnavailable)
    );
}

#[test]
fn directory_creator_transports_target() {
    {
        let mut guard = CAPTURED_TARGET.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    let result =
        directory_creator::create_dir(mock_create_dir_capture_target, "projects/evolution");
    assert_eq!(result, Ok(()));

    let guard = CAPTURED_TARGET.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(guard.as_deref(), Some("projects/evolution"));
}
