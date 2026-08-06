use evo_shell_engine::SetFilesystemScope;

use crate::definitions::contracts::current_directory::CurrentDirectory;
use crate::definitions::domain::entities::shell::Shell;
use crate::definitions::use_cases::initialize_shell::InitializeShellError;

pub fn resolve(
    current_directory: CurrentDirectory,
    set_filesystem_scope: SetFilesystemScope,
) -> Result<Shell, InitializeShellError> {
    let path = current_directory().map_err(InitializeShellError::CurrentDirectory)?;
    let filesystem_scope =
        set_filesystem_scope(path.as_path()).map_err(InitializeShellError::Scope)?;

    Ok(Shell::new(filesystem_scope))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::contracts::current_directory::CurrentDirectoryError;
    use evo_shell_engine::{ScopeError, scope_setter};
    use std::io;
    use std::path::{Path, PathBuf};

    fn current_directory_mock() -> Result<PathBuf, CurrentDirectoryError> {
        std::env::current_dir()
    }

    fn current_directory_error_mock() -> Result<PathBuf, CurrentDirectoryError> {
        Err(io::Error::other("current directory failed"))
    }

    fn scope_error_mock(path: &Path) -> Result<evo_shell_engine::FilesystemScope, ScopeError> {
        Err(ScopeError::NotDirectory(path.to_path_buf()))
    }

    #[test]
    fn shell_resolve_initializes_shell_with_current_directory_and_real_set_scope() {
        let current_dir: CurrentDirectory = current_directory_mock;
        let set_scope: SetFilesystemScope = scope_setter::set;
        let expected = std::env::current_dir().unwrap();

        let shell = resolve(current_dir, set_scope).unwrap();

        assert_eq!(shell.filesystem_scope().path(), expected.as_path());
    }

    #[test]
    fn shell_initialized_by_resolver_owns_expected_filesystem_scope() {
        let shell = resolve(current_directory_mock, scope_setter::set).unwrap();
        let expected = std::env::current_dir().unwrap();

        assert_eq!(shell.filesystem_scope().path(), expected.as_path());
    }

    #[test]
    fn current_directory_error_produces_initialize_shell_error() {
        let result = resolve(current_directory_error_mock, scope_setter::set);

        assert!(matches!(
            result,
            Err(InitializeShellError::CurrentDirectory(_))
        ));
    }

    #[test]
    fn set_filesystem_scope_error_produces_initialize_shell_error() {
        let result = resolve(current_directory_mock, scope_error_mock);

        assert!(matches!(result, Err(InitializeShellError::Scope(_))));
    }

    #[test]
    fn shell_cannot_be_constructed_without_filesystem_scope() {
        let shell = resolve(current_directory_mock, scope_setter::set).unwrap();

        assert!(shell.filesystem_scope().path().is_dir());
    }
}
