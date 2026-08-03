pub mod agents;
pub mod definitions;
pub mod providers;

mod resolvers;

#[cfg(test)]
mod tests {
    use std::io;
    use std::path::Path;

    use crate::agents::scope_setter;
    use crate::definitions::contracts::is_directory::FilesystemError;
    use crate::definitions::use_cases::set_filesystem_scope::{ScopeError, SetFilesystemScope};

    fn always_directory(_: &Path) -> Result<bool, FilesystemError> {
        Ok(true)
    }

    fn never_directory(_: &Path) -> Result<bool, FilesystemError> {
        Ok(false)
    }

    fn provider_error(_: &Path) -> Result<bool, FilesystemError> {
        Err(io::Error::other("provider failed"))
    }

    #[test]
    fn accepted_path_returns_filesystem_scope_with_expected_path() {
        let path = Path::new("/some/path");

        let scope = scope_setter::set(path, always_directory).expect("path should be accepted");

        assert_eq!(scope.path(), path);
    }

    #[test]
    fn rejected_path_returns_scope_error_without_valid_scope() {
        let path = Path::new("/not/a/directory");

        let result = scope_setter::set(path, never_directory);

        assert!(matches!(result, Err(ScopeError::NotDirectory(rejected)) if rejected == path));
    }

    #[test]
    fn provider_error_is_reported_as_filesystem_error() {
        let path = Path::new("/unavailable/path");

        let result = scope_setter::set(path, provider_error);

        assert!(matches!(result, Err(ScopeError::Filesystem(_))));
    }

    #[test]
    fn scope_setter_set_matches_set_filesystem_scope_use_case() {
        let set_scope: SetFilesystemScope = scope_setter::set;

        let scope = set_scope(Path::new("/some/path"), always_directory)
            .expect("agent should match the use case signature");

        assert_eq!(scope.path(), Path::new("/some/path"));
    }
}
