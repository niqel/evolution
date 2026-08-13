use evo_shell::agents::renamer;
use evo_shell::definitions::contracts::rename;
use evo_shell::definitions::use_cases::rename as rename_use_case;
use std::sync::Mutex;

fn mock_rename_success(_target: &str, _new_name: &str) -> Result<(), rename::Error> {
    Ok(())
}

fn mock_rename_unavailable(_target: &str, _new_name: &str) -> Result<(), rename::Error> {
    Err(rename::Error::Unavailable)
}

static CAPTURED_ARGS: Mutex<Option<(String, String)>> = Mutex::new(None);

fn mock_rename_capture_args(target: &str, new_name: &str) -> Result<(), rename::Error> {
    let mut guard = CAPTURED_ARGS.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some((target.to_string(), new_name.to_string()));
    Ok(())
}

static CAPTURED_RESULT: Mutex<Option<Result<(), rename_use_case::Error>>> = Mutex::new(None);

fn mock_final_request(result: Result<(), rename_use_case::Error>) {
    let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(result);
}

#[test]
fn renamer_implements_rename() {
    let rename_op: rename_use_case::Rename = renamer::rename;
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }
    rename_op(
        "videos/gatito.mp4",
        "michi.mp4",
        mock_final_request,
        mock_rename_success,
    );
    {
        let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(*guard, Some(Ok(())));
    }

    let rename_const: rename_use_case::Rename = renamer::RENAME;
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }
    rename_const(
        "videos/gatito.mp4",
        "michi.mp4",
        mock_final_request,
        mock_rename_success,
    );
    {
        let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(*guard, Some(Ok(())));
    }
}

#[test]
fn renamer_success() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    renamer::rename(
        "videos/gatito.mp4",
        "michi.mp4",
        mock_final_request,
        mock_rename_success,
    );

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Ok(())));
}

#[test]
fn renamer_translates_rename_error() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    renamer::rename(
        "videos/gatito.mp4",
        "michi.mp4",
        mock_final_request,
        mock_rename_unavailable,
    );

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Err(rename_use_case::Error::RenameUnavailable)));
}

#[test]
fn renamer_transports_arguments() {
    {
        let mut guard_args = CAPTURED_ARGS.lock().unwrap_or_else(|e| e.into_inner());
        *guard_args = None;
        let mut guard_res = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard_res = None;
    }

    renamer::rename(
        "videos/gatito.mp4",
        "michi.mp4",
        mock_final_request,
        mock_rename_capture_args,
    );

    {
        let guard_res = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(*guard_res, Some(Ok(())));
    }
    {
        let guard_args = CAPTURED_ARGS.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(
            guard_args.as_ref(),
            Some(&("videos/gatito.mp4".to_string(), "michi.mp4".to_string()))
        );
    }
}
