use crate::agents::{shell_initializer, terminal_clearer, welcome_presenter};
use crate::definitions::domain::entities::shell::Shell;
use crate::definitions::use_cases::initialize_shell::InitializeShell;
use crate::definitions::use_cases::starter::StartError;
use crate::definitions::use_cases::terminal_clearer::TerminalClearer;
use crate::definitions::use_cases::welcome_presenter::WelcomePresenter;

pub fn start() -> Result<Shell, StartError> {
    let initialize: InitializeShell = shell_initializer::initialize;
    let clear: TerminalClearer = terminal_clearer::clear;
    let welcome: WelcomePresenter = welcome_presenter::present;

    start_with(initialize, clear, welcome)
}

pub(crate) fn start_with(
    initialize: InitializeShell,
    clear: TerminalClearer,
    welcome: WelcomePresenter,
) -> Result<Shell, StartError> {
    let shell = initialize().map_err(StartError::from)?;
    clear().map_err(StartError::from)?;
    welcome().map_err(StartError::from)?;

    Ok(shell)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::sync::OnceLock;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::SystemTime;

    use evo_shell_engine::scope_setter;

    use crate::definitions::domain::entities::shell::Shell;
    use crate::definitions::use_cases::initialize_shell::InitializeShellError;
    use crate::definitions::use_cases::starter::{Start, StartError};
    use crate::definitions::use_cases::terminal_clearer::TerminalClearError;
    use crate::definitions::use_cases::welcome_presenter::WelcomePresenterError;

    use super::start_with;

    struct TestDirectory {
        path: PathBuf,
    }

    fn shell_from_path(path: &PathBuf) -> Shell {
        Shell::new(scope_setter::set(path.as_path()).unwrap())
    }

    fn temp_directory(prefix: &str) -> TestDirectory {
        let unique = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("system time should be after UNIX_EPOCH")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "evo_shell_{prefix}_{}_{}",
            std::process::id(),
            unique
        ));
        fs::create_dir(&path).expect("temporary test directory should be created");

        TestDirectory { path }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn start_matches_start_function_pointer() {
        let start: Start = start_with;

        let _ = start;
    }

    #[test]
    fn start_with_runs_initialize_clear_then_welcome() {
        static ORDER: AtomicUsize = AtomicUsize::new(0);
        static DIRECTORY: OnceLock<PathBuf> = OnceLock::new();

        fn initialize() -> Result<Shell, InitializeShellError> {
            assert_eq!(ORDER.fetch_add(1, Ordering::SeqCst), 0);
            let path = DIRECTORY.get().expect("directory should be set");
            Ok(shell_from_path(path))
        }

        fn clear() -> Result<(), TerminalClearError> {
            assert_eq!(ORDER.fetch_add(1, Ordering::SeqCst), 1);
            Ok(())
        }

        fn welcome() -> Result<(), WelcomePresenterError> {
            assert_eq!(ORDER.fetch_add(1, Ordering::SeqCst), 2);
            Ok(())
        }

        let directory = temp_directory("starter_order");
        DIRECTORY
            .set(directory.path.clone())
            .expect("directory should be set once");

        let result = start_with(initialize, clear, welcome).unwrap();

        assert_eq!(ORDER.load(Ordering::SeqCst), 3);
        assert!(result.filesystem_scope().path().is_dir());
    }

    #[test]
    fn start_with_stops_before_clear_when_initialize_fails() {
        static ORDER: AtomicUsize = AtomicUsize::new(0);

        fn initialize() -> Result<Shell, InitializeShellError> {
            ORDER.fetch_add(1, Ordering::SeqCst);
            Err(InitializeShellError::CurrentDirectory(
                std::io::Error::other("init failed"),
            ))
        }

        fn clear() -> Result<(), TerminalClearError> {
            ORDER.fetch_add(10, Ordering::SeqCst);
            Ok(())
        }

        fn welcome() -> Result<(), WelcomePresenterError> {
            ORDER.fetch_add(100, Ordering::SeqCst);
            Ok(())
        }

        let result = start_with(initialize, clear, welcome);

        assert!(matches!(result, Err(StartError::InitializeShell(_))));
        assert_eq!(ORDER.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn start_with_stops_before_welcome_when_clear_fails() {
        static ORDER: AtomicUsize = AtomicUsize::new(0);
        static DIRECTORY: OnceLock<PathBuf> = OnceLock::new();

        fn initialize() -> Result<Shell, InitializeShellError> {
            ORDER.fetch_add(1, Ordering::SeqCst);
            let path = DIRECTORY.get().expect("directory should be set");
            Ok(shell_from_path(path))
        }

        fn clear() -> Result<(), TerminalClearError> {
            ORDER.fetch_add(1, Ordering::SeqCst);
            Err(std::io::Error::other("clear failed").into())
        }

        fn welcome() -> Result<(), WelcomePresenterError> {
            ORDER.fetch_add(10, Ordering::SeqCst);
            Ok(())
        }

        let directory = temp_directory("starter_clear_fail");
        DIRECTORY
            .set(directory.path.clone())
            .expect("directory should be set once");

        let result = start_with(initialize, clear, welcome);

        assert!(matches!(result, Err(StartError::TerminalClear(_))));
        assert_eq!(ORDER.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn start_with_reports_welcome_failure_after_clear() {
        static ORDER: AtomicUsize = AtomicUsize::new(0);
        static DIRECTORY: OnceLock<PathBuf> = OnceLock::new();

        fn initialize() -> Result<Shell, InitializeShellError> {
            ORDER.fetch_add(1, Ordering::SeqCst);
            let path = DIRECTORY.get().expect("directory should be set");
            Ok(shell_from_path(path))
        }

        fn clear() -> Result<(), TerminalClearError> {
            ORDER.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }

        fn welcome() -> Result<(), WelcomePresenterError> {
            ORDER.fetch_add(1, Ordering::SeqCst);
            Err(std::io::Error::other("welcome failed").into())
        }

        let directory = temp_directory("starter_welcome_fail");
        DIRECTORY
            .set(directory.path.clone())
            .expect("directory should be set once");

        let result = start_with(initialize, clear, welcome);

        assert!(matches!(result, Err(StartError::WelcomePresent(_))));
        assert_eq!(ORDER.load(Ordering::SeqCst), 3);
    }
}
