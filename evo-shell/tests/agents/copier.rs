use evo_shell::agents::copier;
use evo_shell::definitions::contracts::copy;
use evo_shell::definitions::requesters::transfer_progress_requester;
use evo_shell::definitions::structs::owned::transfer_progress::TransferProgress;
use evo_shell::definitions::use_cases::copy_to;
use std::sync::Mutex;

fn mock_copy_success(
    _progress: transfer_progress_requester::Request,
    _origin: &str,
    _destination: &str,
) -> Result<(), copy::Error> {
    Ok(())
}

fn mock_copy_unavailable(
    _progress: transfer_progress_requester::Request,
    _origin: &str,
    _destination: &str,
) -> Result<(), copy::Error> {
    Err(copy::Error::Unavailable)
}

static CAPTURED_ARGS: Mutex<Option<(String, String)>> = Mutex::new(None);

fn mock_copy_capture_args(
    _progress: transfer_progress_requester::Request,
    origin: &str,
    destination: &str,
) -> Result<(), copy::Error> {
    let mut guard = CAPTURED_ARGS.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some((origin.to_string(), destination.to_string()));
    Ok(())
}

static CAPTURED_PROGRESS: Mutex<Vec<TransferProgress>> = Mutex::new(Vec::new());

fn mock_progress_requester(progress: TransferProgress) {
    let mut guard = CAPTURED_PROGRESS.lock().unwrap_or_else(|e| e.into_inner());
    guard.push(progress);
}

fn mock_copy_with_progress_events(
    progress: transfer_progress_requester::Request,
    _origin: &str,
    _destination: &str,
) -> Result<(), copy::Error> {
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

static CAPTURED_RESULT: Mutex<Option<Result<(), copy_to::Error>>> = Mutex::new(None);

fn mock_final_request(result: Result<(), copy_to::Error>) {
    let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(result);
}

#[test]
fn copier_implements_copy_to() {
    let copy: copy_to::Copy = copier::copy;
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }
    copy(
        "origin.txt",
        "../documents",
        mock_progress_requester,
        mock_final_request,
        mock_copy_success,
    );
    {
        let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(*guard, Some(Ok(())));
    }

    let copy_const: copy_to::Copy = copier::COPY;
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }
    copy_const(
        "origin.txt",
        "../documents",
        mock_progress_requester,
        mock_final_request,
        mock_copy_success,
    );
    {
        let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        assert_eq!(*guard, Some(Ok(())));
    }
}

#[test]
fn copier_success() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    copier::copy(
        "origin.txt",
        "../documents",
        mock_progress_requester,
        mock_final_request,
        mock_copy_success,
    );

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Ok(())));
}

#[test]
fn copier_translates_copy_error() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    copier::copy(
        "origin.txt",
        "../documents",
        mock_progress_requester,
        mock_final_request,
        mock_copy_unavailable,
    );

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Err(copy_to::Error::CopyUnavailable)));
}

#[test]
fn copier_argument_transport_order() {
    {
        let mut guard_args = CAPTURED_ARGS.lock().unwrap_or_else(|e| e.into_inner());
        *guard_args = None;
        let mut guard_res = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard_res = None;
    }

    copier::copy(
        "origin.txt",
        "../documents",
        mock_progress_requester,
        mock_final_request,
        mock_copy_capture_args,
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
fn copier_delivers_progress_events() {
    {
        let mut guard_prog = CAPTURED_PROGRESS.lock().unwrap_or_else(|e| e.into_inner());
        guard_prog.clear();
        let mut guard_res = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard_res = None;
    }

    copier::copy(
        "large_file.iso",
        "/tmp",
        mock_progress_requester,
        mock_final_request,
        mock_copy_with_progress_events,
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
