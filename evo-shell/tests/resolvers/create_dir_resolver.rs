use evo_shell::definitions::contracts::create_dir;
use evo_shell::definitions::use_cases::create_dir as create_dir_use_case;
use evo_shell::resolvers::create_dir_resolver;
use std::sync::Mutex;

fn mock_create_dir_success(_target: &str) -> Result<(), create_dir::Error> {
    Ok(())
}

fn mock_create_dir_unavailable(_target: &str) -> Result<(), create_dir::Error> {
    Err(create_dir::Error::Unavailable)
}

static CAPTURED_RESULT: Mutex<Option<Result<(), create_dir_use_case::Error>>> = Mutex::new(None);

fn mock_request(result: Result<(), create_dir_use_case::Error>) {
    let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    *guard = Some(result);
}

#[test]
fn create_dir_resolver_success() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    create_dir_resolver::resolve(
        mock_create_dir_success,
        "/tmp/evolution/projects/example",
        mock_request,
    );

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(*guard, Some(Ok(())));
}

#[test]
fn create_dir_resolver_translates_error() {
    {
        let mut guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
        *guard = None;
    }

    create_dir_resolver::resolve(
        mock_create_dir_unavailable,
        "/tmp/evolution/projects/example",
        mock_request,
    );

    let guard = CAPTURED_RESULT.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(
        *guard,
        Some(Err(create_dir_use_case::Error::CreateDirUnavailable))
    );
}
