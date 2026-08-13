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

static CAPTURED_RESULT: Mutex<Option<Result<(), trash_use_case::Error>>> = Mutex::new(None);

fn mock_final_request(result: Result<(), trash_use_case::Error>) {
    let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(result);
}

#[test]
fn trasher_implements_trash() {
    let trash_op: trash_use_case::Trash = trasher::trash;
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }
    trash_op("file.txt", mock_final_request, mock_trash_success);
    {
        let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(*guard, Some(Ok(())));
    }

    let trash_const: trash_use_case::Trash = trasher::TRASH;
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }
    trash_const("file.txt", mock_final_request, mock_trash_success);
    {
        let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(*guard, Some(Ok(())));
    }
}

#[test]
fn trasher_success() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    trasher::trash("file.txt", mock_final_request, mock_trash_success);

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Ok(())));
}

#[test]
fn trasher_translates_trash_error() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    trasher::trash("file.txt", mock_final_request, mock_trash_unavailable);

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Err(trash_use_case::Error::TrashUnavailable)));
}

#[test]
fn trasher_transports_target() {
    {
        let mut guard_target = CAPTURED_TARGET.lock().unwrap_or_else(|e| e.into_inner());
        *guard_target = None;
        let mut guard_res = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard_res = None;
    }

    trasher::trash(
        "documents/file.txt",
        mock_final_request,
        mock_trash_capture_target,
    );

    {
        let guard_res = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(*guard_res, Some(Ok(())));
    }
    {
        let guard_target = CAPTURED_TARGET.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(guard_target.as_deref(), Some("documents/file.txt"));
    }
}
