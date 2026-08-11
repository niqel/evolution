use evo_shell::agents::mover;
use evo_shell::definitions::continuations::report_copy_progress;
use evo_shell::definitions::contracts::move_item;
use evo_shell::definitions::structs::copy_progress::CopyProgress;
use evo_shell::definitions::use_cases::move_to;
use evo_shell::handlers::copy_progress_handler;
use std::sync::Mutex;

fn mock_move_success(
    _report: report_copy_progress::Report,
    _origin: &str,
    _destination: &str,
) -> Result<(), move_item::Error> {
    Ok(())
}

fn mock_move_unavailable(
    _report: report_copy_progress::Report,
    _origin: &str,
    _destination: &str,
) -> Result<(), move_item::Error> {
    Err(move_item::Error::Unavailable)
}

static CAPTURED_ARGS: Mutex<Option<(String, String)>> = Mutex::new(None);

fn mock_move_capture_args(
    _report: report_copy_progress::Report,
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
    report: report_copy_progress::Report,
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

fn mock_native_move(
    _report: report_copy_progress::Report,
    _origin: &str,
    _destination: &str,
) -> Result<(), move_item::Error> {
    Ok(())
}

#[test]
fn mover_success() {
    assert_eq!(
        mover::move_to(
            mock_move_success,
            copy_progress_handler::handle,
            "movies/gatito.mp4",
            "../backup"
        ),
        Ok(())
    );
}

#[test]
fn mover_translates_move_error() {
    assert_eq!(
        mover::move_to(
            mock_move_unavailable,
            copy_progress_handler::handle,
            "movies/gatito.mp4",
            "../backup"
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
        copy_progress_handler::handle,
        "movies/gatito.mp4",
        "../backup",
    );
    assert_eq!(result, Ok(()));

    let guard = CAPTURED_ARGS.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(
        guard.as_ref(),
        Some(&("movies/gatito.mp4".to_string(), "../backup".to_string()))
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
        "movies/gatito.mp4",
        "../backup",
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

#[test]
fn mover_allows_move_without_transfer_progress() {
    {
        let mut guard = CAPTURED_PROGRESS.lock().unwrap_or_else(|e| e.into_inner());
        guard.clear();
    }

    let result = mover::move_to(
        mock_native_move,
        mock_progress_handler,
        "movies/gatito.mp4",
        "../backup",
    );
    assert_eq!(result, Ok(()));

    let guard = CAPTURED_PROGRESS.lock().unwrap_or_else(|e| e.into_inner());
    assert!(guard.is_empty());
}
