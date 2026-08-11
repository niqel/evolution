use evo_shell::agents::copier;
use evo_shell::definitions::contracts::copy;
use evo_shell::definitions::use_cases::copy_to;
use std::sync::Mutex;

fn mock_copy_success(_origin: &str, _destination: &str) -> Result<(), copy::Error> {
    Ok(())
}

fn mock_copy_unavailable(_origin: &str, _destination: &str) -> Result<(), copy::Error> {
    Err(copy::Error::Unavailable)
}

static CAPTURED_ARGS: Mutex<Option<(String, String)>> = Mutex::new(None);

fn mock_copy_capture_args(origin: &str, destination: &str) -> Result<(), copy::Error> {
    let mut guard = CAPTURED_ARGS.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some((origin.to_string(), destination.to_string()));
    Ok(())
}

#[test]
fn copier_success() {
    assert_eq!(
        copier::copy(mock_copy_success, "origin.txt", "../documents"),
        Ok(())
    );
}

#[test]
fn copier_translates_copy_error() {
    assert_eq!(
        copier::copy(mock_copy_unavailable, "origin.txt", "../documents"),
        Err(copy_to::Error::CopyUnavailable)
    );
}

#[test]
fn copier_argument_transport_order() {
    {
        let mut guard = CAPTURED_ARGS.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    let result = copier::copy(mock_copy_capture_args, "origin.txt", "../documents");
    assert_eq!(result, Ok(()));

    let guard = CAPTURED_ARGS.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(
        guard.as_ref(),
        Some(&("origin.txt".to_string(), "../documents".to_string()))
    );
}
