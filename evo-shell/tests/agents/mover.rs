use evo_shell::agents::mover;
use evo_shell::definitions::contracts::move_item;
use evo_shell::definitions::requesters::transfer_progress_requester;
use evo_shell::definitions::structs::transfer_progress::TransferProgress;
use evo_shell::definitions::use_cases::move_to;
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

static CAPTURED_ARGS: Mutex<Option<(String, String)>> = Mutex::new(None);

fn mock_move_capture_args(
    _progress: transfer_progress_requester::Request,
    origin: &str,
    destination: &str,
) -> Result<(), move_item::Error> {
    let mut guard = CAPTURED_ARGS.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some((origin.to_string(), destination.to_string()));
    Ok(())
}

static CAPTURED_PROGRESS: Mutex<Vec<TransferProgress>> = Mutex::new(Vec::new());

fn mock_progress_requester(progress: TransferProgress) {
    let mut guard = CAPTURED_PROGRESS.lock().unwrap_or_else(|e| e.into_inner());
    guard.push(progress);
}

fn mock_move_with_progress_events(
    progress: transfer_progress_requester::Request,
    _origin: &str,
    _destination: &str,
) -> Result<(), move_item::Error> {
    progress(TransferProgress {
        total_bytes: Some(1000),
        transferred_bytes: 0,
    });
    progress(TransferProgress {
        total_bytes: Some(1000),
        transferred_bytes: 500,
    });
    progress(TransferProgress {
        total_bytes: Some(1000),
        transferred_bytes: 1000,
    });
    Ok(())
}

static CAPTURED_RESULT: Mutex<Option<Result<(), move_to::Error>>> = Mutex::new(None);

fn mock_final_request(result: Result<(), move_to::Error>) {
    let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(result);
}

#[test]
fn mover_implements_move_to() {
    let move_operation: move_to::Move = mover::move_to;
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }
    move_operation(
        "origin.txt",
        "../documents",
        mock_progress_requester,
        mock_final_request,
        mock_move_success,
    );
    {
        let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(*guard, Some(Ok(())));
    }

    let move_const: move_to::Move = mover::MOVE;
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }
    move_const(
        "origin.txt",
        "../documents",
        mock_progress_requester,
        mock_final_request,
        mock_move_success,
    );
    {
        let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(*guard, Some(Ok(())));
    }
}

#[test]
fn mover_success() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    mover::move_to(
        "origin.txt",
        "../documents",
        mock_progress_requester,
        mock_final_request,
        mock_move_success,
    );

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Ok(())));
}

#[test]
fn mover_translates_move_error() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    mover::move_to(
        "origin.txt",
        "../documents",
        mock_progress_requester,
        mock_final_request,
        mock_move_unavailable,
    );

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Err(move_to::Error::MoveUnavailable)));
}

#[test]
fn mover_transports_arguments() {
    {
        let mut guard_args = CAPTURED_ARGS.lock().unwrap_or_else(|e| e.into_inner());
        *guard_args = None;
        let mut guard_res = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard_res = None;
    }

    mover::move_to(
        "origin.txt",
        "../documents",
        mock_progress_requester,
        mock_final_request,
        mock_move_capture_args,
    );

    {
        let guard_res = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(*guard_res, Some(Ok(())));
    }
    {
        let guard_args = CAPTURED_ARGS.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(
            guard_args.as_ref(),
            Some(&("origin.txt".to_string(), "../documents".to_string()))
        );
    }
}

#[test]
fn mover_delivers_transfer_progress() {
    {
        let mut guard_prog = CAPTURED_PROGRESS.lock().unwrap_or_else(|e| e.into_inner());
        guard_prog.clear();
        let mut guard_res = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard_res = None;
    }

    mover::move_to(
        "large_file.iso",
        "/tmp",
        mock_progress_requester,
        mock_final_request,
        mock_move_with_progress_events,
    );

    {
        let guard_res = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(*guard_res, Some(Ok(())));
    }

    let guard_prog = CAPTURED_PROGRESS.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(
        *guard_prog,
        vec![
            TransferProgress {
                total_bytes: Some(1000),
                transferred_bytes: 0,
            },
            TransferProgress {
                total_bytes: Some(1000),
                transferred_bytes: 500,
            },
            TransferProgress {
                total_bytes: Some(1000),
                transferred_bytes: 1000,
            },
        ]
    );
}
