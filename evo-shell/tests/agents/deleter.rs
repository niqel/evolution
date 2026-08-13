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

static CAPTURED_RESULT: Mutex<Option<Result<(), delete_use_case::Error>>> = Mutex::new(None);

fn mock_final_request(result: Result<(), delete_use_case::Error>) {
    let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(result);
}

#[test]
fn deleter_implements_delete() {
    let delete_op: delete_use_case::Delete = deleter::delete;
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }
    delete_op("file.txt", mock_final_request, mock_delete_success);
    {
        let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(*guard, Some(Ok(())));
    }

    let delete_const: delete_use_case::Delete = deleter::DELETE;
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }
    delete_const("file.txt", mock_final_request, mock_delete_success);
    {
        let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(*guard, Some(Ok(())));
    }
}

#[test]
fn deleter_success() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    deleter::delete("file.txt", mock_final_request, mock_delete_success);

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Ok(())));
}

#[test]
fn deleter_translates_delete_error() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    deleter::delete("file.txt", mock_final_request, mock_delete_unavailable);

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Err(delete_use_case::Error::DeleteUnavailable)));
}

#[test]
fn deleter_transports_target() {
    {
        let mut guard_target = CAPTURED_TARGET.lock().unwrap_or_else(|e| e.into_inner());
        *guard_target = None;
        let mut guard_res = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard_res = None;
    }

    deleter::delete(
        "documents/file.txt",
        mock_final_request,
        mock_delete_capture_target,
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
