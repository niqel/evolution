pub mod agents;
pub mod definitions;
pub mod providers;

mod resolvers;

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::fs;
    use std::fs::ReadDir;
    use std::io;
    use std::path::Path;
    use std::path::PathBuf;
    use std::time::SystemTime;

    use crate::agents::iterator;
    use crate::agents::scope_setter;
    use crate::definitions::contracts::is_directory::FilesystemError;
    use crate::definitions::contracts::next_directory_entry::NextDirectoryEntry;
    use crate::definitions::contracts::read_directory::ReadDirectory;
    use crate::definitions::domain::entities::filesystem_entry::FilesystemEntryKind;
    use crate::definitions::domain::entities::filesystem_iteration::FilesystemIteration;
    use crate::definitions::use_cases::iter::{Iter, IterError};
    use crate::definitions::use_cases::set_filesystem_scope::{ScopeError, SetFilesystemScope};
    use crate::providers;
    use crate::resolvers::filesystem_entry;

    struct TestDirectory {
        path: PathBuf,
    }

    impl TestDirectory {
        fn new(name: &str) -> Self {
            let unique = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("system time should be after UNIX_EPOCH")
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "evo_shell_engine_{name}_{}_{}",
                std::process::id(),
                unique
            ));

            fs::create_dir(&path).expect("temporary test directory should be created");

            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn always_directory(_: &Path) -> Result<bool, FilesystemError> {
        Ok(true)
    }

    fn never_directory(_: &Path) -> Result<bool, FilesystemError> {
        Ok(false)
    }

    fn provider_error(_: &Path) -> Result<bool, FilesystemError> {
        Err(io::Error::other("provider failed"))
    }

    fn read_directory_error(_: &Path) -> Result<ReadDir, io::Error> {
        Err(io::Error::other("open failed"))
    }

    fn next_directory_entry_error(
        _: &mut FilesystemIteration,
    ) -> Result<Option<fs::DirEntry>, io::Error> {
        Err(io::Error::other("next failed"))
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

    #[test]
    fn iter_accepts_valid_filesystem_scope_and_returns_iteration() {
        let directory = TestDirectory::new("iter_accepts_scope");
        let scope = scope_setter::set(directory.path(), providers::is_directory::provide).unwrap();

        let result = iterator::iter(&scope, providers::read_directory::provide);

        assert!(result.is_ok());
    }

    #[test]
    fn iterator_iter_matches_iter_use_case_function_pointer() {
        let directory = TestDirectory::new("iter_use_case_pointer");
        let scope = scope_setter::set(directory.path(), providers::is_directory::provide).unwrap();
        let iter_scope: Iter = iterator::iter;

        let result = iter_scope(&scope, providers::read_directory::provide);

        assert!(result.is_ok());
    }

    #[test]
    fn read_directory_contract_matches_provider_function_pointer() {
        let directory = TestDirectory::new("read_directory_contract_pointer");
        let read_directory: ReadDirectory = providers::read_directory::provide;

        let result = read_directory(directory.path());

        assert!(result.is_ok());
    }

    #[test]
    fn next_directory_entry_contract_matches_provider_function_pointer() {
        let directory = TestDirectory::new("next_directory_entry_contract_pointer");
        let scope = scope_setter::set(directory.path(), providers::is_directory::provide).unwrap();
        let mut iteration = iterator::iter(&scope, providers::read_directory::provide).unwrap();
        let next_directory_entry: NextDirectoryEntry = providers::next_directory_entry::provide;

        let result = next_directory_entry(&mut iteration);

        assert!(matches!(result, Ok(None)));
    }

    #[test]
    fn filesystem_entry_resolve_returns_one_entry_per_call_and_reaches_end() {
        let directory = TestDirectory::new("entry_one_per_call");
        fs::write(directory.path().join("report.txt"), "report").unwrap();
        fs::create_dir(directory.path().join("images")).unwrap();
        let scope = scope_setter::set(directory.path(), providers::is_directory::provide).unwrap();
        let mut iteration = iterator::iter(&scope, providers::read_directory::provide).unwrap();

        let first =
            filesystem_entry::resolve(&mut iteration, providers::next_directory_entry::provide)
                .unwrap();
        let second =
            filesystem_entry::resolve(&mut iteration, providers::next_directory_entry::provide)
                .unwrap();
        let end =
            filesystem_entry::resolve(&mut iteration, providers::next_directory_entry::provide)
                .unwrap();

        assert!(first.is_some());
        assert!(second.is_some());
        assert!(end.is_none());
    }

    #[test]
    fn filesystem_entry_resolve_distinguishes_file_and_directory() {
        let directory = TestDirectory::new("entry_kind");
        fs::write(directory.path().join("report.txt"), "report").unwrap();
        fs::create_dir(directory.path().join("images")).unwrap();
        let scope = scope_setter::set(directory.path(), providers::is_directory::provide).unwrap();
        let mut iteration = iterator::iter(&scope, providers::read_directory::provide).unwrap();
        let mut found_file = false;
        let mut found_directory = false;

        while let Some(entry) =
            filesystem_entry::resolve(&mut iteration, providers::next_directory_entry::provide)
                .unwrap()
        {
            if entry.name() == OsStr::new("report.txt") {
                assert_eq!(entry.path(), directory.path().join("report.txt").as_path());
                assert_eq!(entry.kind(), FilesystemEntryKind::File);
                found_file = true;
            }

            if entry.name() == OsStr::new("images") {
                assert_eq!(entry.path(), directory.path().join("images").as_path());
                assert_eq!(entry.kind(), FilesystemEntryKind::Directory);
                found_directory = true;
            }
        }

        assert!(found_file);
        assert!(found_directory);
    }

    #[cfg(unix)]
    #[test]
    fn filesystem_entry_resolve_distinguishes_symlink() {
        use std::os::unix::fs::symlink;

        let directory = TestDirectory::new("entry_symlink");
        fs::write(directory.path().join("report.txt"), "report").unwrap();
        symlink(
            directory.path().join("report.txt"),
            directory.path().join("report-link.txt"),
        )
        .unwrap();
        let scope = scope_setter::set(directory.path(), providers::is_directory::provide).unwrap();
        let mut iteration = iterator::iter(&scope, providers::read_directory::provide).unwrap();
        let mut found_symlink = false;

        while let Some(entry) =
            filesystem_entry::resolve(&mut iteration, providers::next_directory_entry::provide)
                .unwrap()
        {
            if entry.name() == OsStr::new("report-link.txt") {
                assert_eq!(entry.kind(), FilesystemEntryKind::Symlink);
                found_symlink = true;
            }
        }

        assert!(found_symlink);
    }

    #[test]
    fn iterator_iter_reports_open_directory_error() {
        let directory = TestDirectory::new("open_error");
        let scope = scope_setter::set(directory.path(), always_directory).unwrap();

        let result = iterator::iter(&scope, read_directory_error);

        assert!(matches!(result, Err(IterError::OpenDirectory(_))));
    }

    #[test]
    fn filesystem_entry_resolve_reports_next_entry_error() {
        let directory = TestDirectory::new("next_error");
        let scope = scope_setter::set(directory.path(), providers::is_directory::provide).unwrap();
        let mut iteration = iterator::iter(&scope, providers::read_directory::provide).unwrap();

        let result = filesystem_entry::resolve(&mut iteration, next_directory_entry_error);

        assert!(matches!(result, Err(IterError::NextEntry(_))));
    }
}
