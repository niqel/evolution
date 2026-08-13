use evo_shell::definitions::contracts::move_item;
use evo_shell::definitions::requesters::transfer_progress_requester;
use evo_shell::definitions::structs::transfer_progress::TransferProgress;
use evo_shell::definitions::use_cases::move_to;
use evo_shell::resolvers::move_resolver;
use std::sync::Mutex;

fn mock_move_success(
    _progress: transfer_progress_requester::Request,
    _origin: &str,
    _destination: &str,
) -> Result<(), move_item::Error> {
    Ok(())
}

fn mock_move_unavailable(
    _progress: transfer_progress_requester::Request,
    _origin: &str,
    _destination: &str,
) -> Result<(), move_item::Error> {
    Err(move_item::Error::Unavailable)
}

static CAPTURED_PROGRESS: Mutex<Vec<TransferProgress>> = Mutex::new(Vec::new());

fn mock_progress_requester(progress: TransferProgress) {
    let mut guard = CAPTURED_PROGRESS.lock().unwrap_or_else(|e| e.into_inner());
    guard.push(progress);
}

fn mock_move_with_progress(
    progress: transfer_progress_requester::Request,
    _origin: &str,
    _destination: &str,
) -> Result<(), move_item::Error> {
    progress(TransferProgress {
        total_bytes: Some(100),
        transferred_bytes: 50,
    });
    Ok(())
}

static CAPTURED_RESULT: Mutex<Option<Result<(), move_to::Error>>> = Mutex::new(None);

fn mock_final_request(result: Result<(), move_to::Error>) {
    let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(result);
}

#[test]
fn move_resolver_success() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    move_resolver::resolve(
        mock_move_success,
        "origin.txt",
        "../documents",
        mock_progress_requester,
        mock_final_request,
    );

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Ok(())));
}

#[test]
fn move_resolver_translates_error() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    move_resolver::resolve(
        mock_move_unavailable,
        "origin.txt",
        "../documents",
        mock_progress_requester,
        mock_final_request,
    );

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Err(move_to::Error::MoveUnavailable)));
}

#[test]
fn move_resolver_transports_progress_requester() {
    {
        let mut guard_prog = CAPTURED_PROGRESS.lock().unwrap_or_else(|e| e.into_inner());
        guard_prog.clear();
        let mut guard_res = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard_res = None;
    }

    move_resolver::resolve(
        mock_move_with_progress,
        "origin.txt",
        "../documents",
        mock_progress_requester,
        mock_final_request,
    );

    {
        let guard_res = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(*guard_res, Some(Ok(())));
    }

    let guard_prog = CAPTURED_PROGRESS.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(
        *guard_prog,
        vec![TransferProgress {
            total_bytes: Some(100),
            transferred_bytes: 50,
        }]
    );
}
