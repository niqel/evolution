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
static CAPTURED_RESULT: Mutex<Option<Result<(), create_dir_use_case::Error>>> = Mutex::new(None);

fn mock_create_dir_capture_target(target: &str) -> Result<(), create_dir::Error> {
    let mut guard = CAPTURED_TARGET.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(target.to_string());
    Ok(())
}

fn mock_request(result: Result<(), create_dir_use_case::Error>) {
    let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(result);
}

#[test]
fn directory_creator_success() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    directory_creator::create(
        mock_create_dir_success,
        "/tmp/evolution/projects/example",
        mock_request,
    );

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Ok(())));
}

#[test]
fn directory_creator_translates_error() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    directory_creator::create(
        mock_create_dir_unavailable,
        "/tmp/evolution/projects/example",
        mock_request,
    );

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(
        *guard,
        Some(Err(create_dir_use_case::Error::CreateDirUnavailable))
    );
}

#[test]
fn directory_creator_transports_target() {
    {
        let mut guard_target = CAPTURED_TARGET.lock().unwrap_or_else(|e| e.into_inner());
        *guard_target = None;
        let mut guard_result = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard_result = None;
    }

    directory_creator::create(
        mock_create_dir_capture_target,
        "/tmp/evolution/projects/example",
        mock_request,
    );

    let guard_target = CAPTURED_TARGET.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(
        guard_target.as_deref(),
        Some("/tmp/evolution/projects/example")
    );

    let guard_result = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard_result, Some(Ok(())));
}
