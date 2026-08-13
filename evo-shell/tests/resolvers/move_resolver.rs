use evo_shell::definitions::contracts::move_item;
use evo_shell::definitions::requesters::copy_progress_requester;
use evo_shell::definitions::structs::copy_progress::CopyProgress;
use evo_shell::resolvers::move_resolver;
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

static CAPTURED_PROGRESS: Mutex<Vec<CopyProgress>> = Mutex::new(Vec::new());

fn mock_progress_handler(progress: CopyProgress) {
    let mut guard = CAPTURED_PROGRESS.lock().unwrap_or_else(|e| e.into_inner());
    guard.push(progress);
}

fn mock_move_with_progress(
    report: copy_progress_requester::Request,
    _origin: &str,
    _destination: &str,
) -> Result<(), move_item::Error> {
    report(CopyProgress {
        total_bytes: Some(100),
        copied_bytes: 50,
    });
    Ok(())
}

fn dummy_progress_handler(_progress: CopyProgress) {}

#[test]
fn move_resolver_success() {
    assert_eq!(
        move_resolver::resolve(
            mock_move_success,
            dummy_progress_handler,
            "origin.txt",
            "../documents"
        ),
        Ok(())
    );
}

#[test]
fn move_resolver_translates_error() {
    assert_eq!(
        move_resolver::resolve(
            mock_move_unavailable,
            dummy_progress_handler,
            "origin.txt",
            "../documents"
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
        mock_move_with_progress,
        mock_progress_handler,
        "origin.txt",
        "../documents",
    );
    assert_eq!(result, Ok(()));

    let guard = CAPTURED_PROGRESS.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(
        *guard,
        vec![CopyProgress {
            total_bytes: Some(100),
            copied_bytes: 50,
        }]
    );
}
