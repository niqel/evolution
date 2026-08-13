use evo_shell::definitions::contracts::copy;
use evo_shell::definitions::requesters::copy_progress_requester;
use evo_shell::definitions::structs::copy_progress::CopyProgress;
use evo_shell::definitions::use_cases::copy_to;
use evo_shell::resolvers::copy_resolver;
use std::sync::Mutex;

fn mock_copy_success(
    _progress: copy_progress_requester::Request,
    _origin: &str,
    _destination: &str,
) -> Result<(), copy::Error> {
    Ok(())
}

fn mock_copy_unavailable(
    _progress: copy_progress_requester::Request,
    _origin: &str,
    _destination: &str,
) -> Result<(), copy::Error> {
    Err(copy::Error::Unavailable)
}

static CAPTURED_PROGRESS: Mutex<Vec<CopyProgress>> = Mutex::new(Vec::new());

fn mock_progress_requester(progress: CopyProgress) {
    let mut guard = CAPTURED_PROGRESS.lock().unwrap_or_else(|e| e.into_inner());
    guard.push(progress);
}

fn mock_copy_with_progress(
    progress: copy_progress_requester::Request,
    _origin: &str,
    _destination: &str,
) -> Result<(), copy::Error> {
    progress(CopyProgress {
        total_bytes: Some(100),
        copied_bytes: 50,
    });
    Ok(())
}

static CAPTURED_RESULT: Mutex<Option<Result<(), copy_to::Error>>> = Mutex::new(None);

fn mock_final_request(result: Result<(), copy_to::Error>) {
    let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(result);
}

#[test]
fn copy_resolver_success() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    copy_resolver::resolve(
        mock_copy_success,
        "origin.txt",
        "../documents",
        mock_progress_requester,
        mock_final_request,
    );

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Ok(())));
}

#[test]
fn copy_resolver_translates_error() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    copy_resolver::resolve(
        mock_copy_unavailable,
        "origin.txt",
        "../documents",
        mock_progress_requester,
        mock_final_request,
    );

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Err(copy_to::Error::CopyUnavailable)));
}

#[test]
fn copy_resolver_transports_progress_requester() {
    {
        let mut guard_prog = CAPTURED_PROGRESS.lock().unwrap_or_else(|e| e.into_inner());
        guard_prog.clear();
        let mut guard_res = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard_res = None;
    }

    copy_resolver::resolve(
        mock_copy_with_progress,
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
        vec![CopyProgress {
            total_bytes: Some(100),
            copied_bytes: 50,
        }]
    );
}
