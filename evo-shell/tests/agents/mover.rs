use evo_shell::agents::mover;
use evo_shell::definitions::contracts::move_item;
use evo_shell::definitions::requesters::copy_progress_requester;
use evo_shell::definitions::structs::copy_progress::CopyProgress;
use evo_shell::definitions::use_cases::move_to;
use std::sync::Mutex;

fn mock_move_success(
    _report: copy_progress_requester::Request,
    _origin: &str,
    _destination: &str,
) -> Result<(), move_item::Error> {
    Ok(())
}

fn mock_move_unavailable(
    _report: copy_progress_requester::Request,
    _origin: &str,
    _destination: &str,
) -> Result<(), move_item::Error> {
    Err(move_item::Error::Unavailable)
}

static CAPTURED_ARGS: Mutex<Option<(String, String)>> = Mutex::new(None);

fn mock_move_capture_args(
    _report: copy_progress_requester::Request,
    origin: &str,
    destination: &str,
) -> Result<(), move_item::Error> {
    let mut guard = CAPTURED_ARGS.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some((origin.to_string(), destination.to_string()));
    Ok(())
}

static CAPTURED_PROGRESS: Mutex<Vec<CopyProgress>> = Mutex::new(Vec::new());

fn mock_progress_handler(progress: CopyProgress) {
    let mut guard = CAPTURED_PROGRESS.lock().unwrap_or_else(|e| e.into_inner());
    guard.push(progress);
}

fn mock_move_with_progress_events(
    report: copy_progress_requester::Request,
    _origin: &str,
    _destination: &str,
) -> Result<(), move_item::Error> {
    report(CopyProgress {
        total_bytes: Some(1000),
        copied_bytes: 0,
    });
    report(CopyProgress {
        total_bytes: Some(1000),
        copied_bytes: 500,
    });
    report(CopyProgress {
        total_bytes: Some(1000),
        copied_bytes: 1000,
    });
    Ok(())
}

fn dummy_progress_handler(_progress: CopyProgress) {}

#[test]
fn mover_allows_move_without_transfer_progress() {
    assert_eq!(
        mover::move_to(
            mock_move_success,
            dummy_progress_handler,
            "origin.txt",
            "../documents"
        ),
        Ok(())
    );
}

#[test]
fn mover_translates_move_error() {
    assert_eq!(
        mover::move_to(
            mock_move_unavailable,
            dummy_progress_handler,
            "origin.txt",
            "../documents"
        ),
        Err(move_to::Error::MoveUnavailable)
    );
}

#[test]
fn mover_transports_arguments() {
    {
        let mut guard = CAPTURED_ARGS.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    let result = mover::move_to(
        mock_move_capture_args,
        dummy_progress_handler,
        "origin.txt",
        "../documents",
    );
    assert_eq!(result, Ok(()));

    let guard = CAPTURED_ARGS.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(
        guard.as_ref(),
        Some(&("origin.txt".to_string(), "../documents".to_string()))
    );
}

#[test]
fn mover_delivers_transfer_progress() {
    {
        let mut guard = CAPTURED_PROGRESS.lock().unwrap_or_else(|e| e.into_inner());
        guard.clear();
    }

    let result = mover::move_to(
        mock_move_with_progress_events,
        mock_progress_handler,
        "large_file.iso",
        "/tmp",
    );
    assert_eq!(result, Ok(()));

    let guard = CAPTURED_PROGRESS.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(
        *guard,
        vec![
            CopyProgress {
                total_bytes: Some(1000),
                copied_bytes: 0,
            },
            CopyProgress {
                total_bytes: Some(1000),
                copied_bytes: 500,
            },
            CopyProgress {
                total_bytes: Some(1000),
                copied_bytes: 1000,
            },
        ]
    );
}
