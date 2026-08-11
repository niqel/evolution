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

#[test]
fn renamer_success() {
    assert_eq!(
        renamer::rename(mock_rename_success, "videos/gatito.mp4", "michi.mp4"),
        Ok(())
    );
}

#[test]
fn renamer_translates_rename_error() {
    assert_eq!(
        renamer::rename(mock_rename_unavailable, "videos/gatito.mp4", "michi.mp4"),
        Err(rename_use_case::Error::RenameUnavailable)
    );
}

#[test]
fn renamer_transports_arguments() {
    {
        let mut guard = CAPTURED_ARGS.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    let result = renamer::rename(mock_rename_capture_args, "videos/gatito.mp4", "michi.mp4");
    assert_eq!(result, Ok(()));

    let guard = CAPTURED_ARGS.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(
        guard.as_ref(),
        Some(&("videos/gatito.mp4".to_string(), "michi.mp4".to_string()))
    );
}
