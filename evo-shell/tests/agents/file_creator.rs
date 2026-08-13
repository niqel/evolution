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

static CAPTURED_RESULT: Mutex<Option<Result<(), create_file_use_case::Error>>> = Mutex::new(None);

fn mock_final_request(result: Result<(), create_file_use_case::Error>) {
    let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(result);
}

#[test]
fn file_creator_implements_create_file() {
    let create: create_file_use_case::CreateFile = file_creator::create_file;
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }
    create("notes.txt", mock_final_request, mock_create_file_success);
    {
        let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(*guard, Some(Ok(())));
    }

    let create_const: create_file_use_case::CreateFile = file_creator::CREATE;
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }
    create_const("notes.txt", mock_final_request, mock_create_file_success);
    {
        let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(*guard, Some(Ok(())));
    }
}

#[test]
fn file_creator_success() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    file_creator::create_file("notes.txt", mock_final_request, mock_create_file_success);

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Ok(())));
}

#[test]
fn file_creator_translates_error() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    file_creator::create_file(
        "notes.txt",
        mock_final_request,
        mock_create_file_unavailable,
    );

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(
        *guard,
        Some(Err(create_file_use_case::Error::CreateFileUnavailable))
    );
}

#[test]
fn file_creator_transports_target() {
    {
        let mut guard_target = CAPTURED_TARGET.lock().unwrap_or_else(|e| e.into_inner());
        *guard_target = None;
        let mut guard_res = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard_res = None;
    }

    file_creator::create_file(
        "documents/notes.txt",
        mock_final_request,
        mock_create_file_capture_target,
    );

    {
        let guard_res = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(*guard_res, Some(Ok(())));
    }
    {
        let guard_target = CAPTURED_TARGET.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(guard_target.as_deref(), Some("documents/notes.txt"));
    }
}
