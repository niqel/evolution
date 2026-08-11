use evo_shell::definitions::continuations::report_copy_progress;
use evo_shell::definitions::contracts::move_item;
use evo_shell::definitions::structs::copy_progress::CopyProgress;
use evo_shell::handlers::copy_progress_handler;
use evo_shell::resolvers::move_resolver;
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

static CAPTURED_PROGRESS: Mutex<Vec<CopyProgress>> = Mutex::new(Vec::new());

fn mock_progress_handler(progress: CopyProgress) {
    let mut guard = CAPTURED_PROGRESS.lock().unwrap_or_else(|e| e.into_inner());
    guard.push(progress);
}

fn mock_move_emits_progress(
    report: report_copy_progress::Report,
    _origin: &str,
    _destination: &str,
) -> Result<(), move_item::Error> {
    report(CopyProgress {
        total_bytes: Some(500),
        copied_bytes: 250,
    });
    Ok(())
}

#[test]
fn move_resolver_success() {
    assert_eq!(
        move_resolver::resolve(
            mock_move_success,
            copy_progress_handler::handle,
            "movies/gatito.mp4",
            "../backup"
        ),
        Ok(())
    );
}

#[test]
fn move_resolver_translates_error() {
    assert_eq!(
        move_resolver::resolve(
            mock_move_unavailable,
            copy_progress_handler::handle,
            "movies/gatito.mp4",
            "../backup"
        ),
        Err(move_resolver::Error::Unavailable)
    );
}

#[test]
fn move_resolver_transports_progress_reporter() {
    {
        let mut guard = CAPTURED_PROGRESS.lock().unwrap_or_else(|e| e.into_inner());
        guard.clear();
    }

    let result = move_resolver::resolve(
        mock_move_emits_progress,
        mock_progress_handler,
        "movies/gatito.mp4",
        "../backup",
    );
    assert_eq!(result, Ok(()));

    let guard = CAPTURED_PROGRESS.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(
        *guard,
        vec![CopyProgress {
            total_bytes: Some(500),
            copied_bytes: 250,
        }]
    );
}
