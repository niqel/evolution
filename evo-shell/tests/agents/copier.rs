use evo_shell::agents::copier;
use evo_shell::definitions::continuations::report_copy_progress;
use evo_shell::definitions::contracts::copy;
use evo_shell::definitions::structs::copy_progress::CopyProgress;
use evo_shell::definitions::use_cases::copy_to;
use evo_shell::handlers::copy_progress_handler;
use std::sync::Mutex;

fn mock_copy_success(
    _report: report_copy_progress::Report,
    _origin: &str,
    _destination: &str,
) -> Result<(), copy::Error> {
    Ok(())
}

fn mock_copy_unavailable(
    _report: report_copy_progress::Report,
    _origin: &str,
    _destination: &str,
) -> Result<(), copy::Error> {
    Err(copy::Error::Unavailable)
}

static CAPTURED_ARGS: Mutex<Option<(String, String)>> = Mutex::new(None);

fn mock_copy_capture_args(
    _report: report_copy_progress::Report,
    origin: &str,
    destination: &str,
) -> Result<(), copy::Error> {
    let mut guard = CAPTURED_ARGS.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some((origin.to_string(), destination.to_string()));
    Ok(())
}

static CAPTURED_PROGRESS: Mutex<Vec<CopyProgress>> = Mutex::new(Vec::new());

fn mock_progress_handler(progress: CopyProgress) {
    let mut guard = CAPTURED_PROGRESS.lock().unwrap_or_else(|e| e.into_inner());
    guard.push(progress);
}

fn mock_copy_with_progress_events(
    report: report_copy_progress::Report,
    _origin: &str,
    _destination: &str,
) -> Result<(), copy::Error> {
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

#[test]
fn copier_success() {
    assert_eq!(
        copier::copy(
            mock_copy_success,
            copy_progress_handler::handle,
            "origin.txt",
            "../documents"
        ),
        Ok(())
    );
}

#[test]
fn copier_translates_copy_error() {
    assert_eq!(
        copier::copy(
            mock_copy_unavailable,
            copy_progress_handler::handle,
            "origin.txt",
            "../documents"
        ),
        Err(copy_to::Error::CopyUnavailable)
    );
}

#[test]
fn copier_argument_transport_order() {
    {
        let mut guard = CAPTURED_ARGS.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    let result = copier::copy(
        mock_copy_capture_args,
        copy_progress_handler::handle,
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
fn copier_delivers_progress_events() {
    {
        let mut guard = CAPTURED_PROGRESS.lock().unwrap_or_else(|e| e.into_inner());
        guard.clear();
    }

    let result = copier::copy(
        mock_copy_with_progress_events,
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
