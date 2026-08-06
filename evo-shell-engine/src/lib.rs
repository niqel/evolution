mod agents;
mod definitions;
mod providers;

mod resolvers;

pub use agents::{enterer, filterer, iteration_advancer, iterator, scope_setter, selector};
pub use definitions::domain::entities::filesystem_entry::{FilesystemEntry, FilesystemEntryKind};
pub use definitions::domain::entities::filesystem_iteration::FilesystemIteration;
pub use definitions::domain::entities::filesystem_iteration_item::FilesystemIterationItem;
pub use definitions::domain::entities::filesystem_scope::FilesystemScope;
pub use definitions::domain::value_objects::filter::{
    FilterComparison, FilterExpression, FilterOperand, FilterOperator, FilterProperty, FilterValue,
};
pub use definitions::domain::value_objects::select::{
    ProjectedRow, ProjectedValue, SelectProperty, StructuredProjection,
};
pub use definitions::use_cases::advance::Advance;
pub use definitions::use_cases::enter::Enter;
pub use definitions::use_cases::filter::{Filter, FilterError};
pub use definitions::use_cases::iter::{Iter, IterError};
pub use definitions::use_cases::select::{Select, SelectError};
pub use definitions::use_cases::set_filesystem_scope::{ScopeError, SetFilesystemScope};

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::fs;
    use std::fs::ReadDir;
    use std::io;
    use std::path::Path;
    use std::path::PathBuf;
    use std::time::SystemTime;

    use crate::agents::enterer;
    use crate::agents::iteration_advancer;
    use crate::agents::iterator;
    use crate::agents::scope_setter;
    use crate::definitions::contracts::is_directory::FilesystemError;
    use crate::definitions::contracts::next_directory_entry::NextDirectoryEntry;
    use crate::definitions::contracts::read_directory::ReadDirectory;
    use crate::definitions::domain::entities::filesystem_iteration::FilesystemIteration;
    use crate::providers;
    use crate::resolvers::filesystem_entry;
    use crate::resolvers::filesystem_iteration;
    use crate::resolvers::filesystem_path;
    use crate::resolvers::filesystem_scope;
    use crate::{
        Advance, Enter, FilesystemEntryKind, FilesystemIterationItem, Iter, IterError, ScopeError,
        SetFilesystemScope,
    };

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

    static REMOVED_DIRECTORY_ENTRY: std::sync::Mutex<Option<fs::DirEntry>> =
        std::sync::Mutex::new(None);

    fn removed_directory_entry(
        _: &mut FilesystemIteration,
    ) -> Result<Option<fs::DirEntry>, io::Error> {
        Ok(REMOVED_DIRECTORY_ENTRY.lock().unwrap().take())
    }

    #[test]
    fn accepted_path_returns_filesystem_scope_with_expected_path() {
        let directory = TestDirectory::new("accepted_path");
        let path = directory.path();

        let scope =
            filesystem_scope::resolve(path, always_directory).expect("path should be accepted");

        assert_eq!(scope.path(), path.canonicalize().unwrap().as_path());
    }

    #[test]
    fn rejected_path_returns_scope_error_without_valid_scope() {
        let directory = TestDirectory::new("rejected_path");
        let path = directory.path();

        let result = filesystem_scope::resolve(path, never_directory);

        assert!(
            matches!(result, Err(ScopeError::NotDirectory(rejected)) if rejected == path.canonicalize().unwrap())
        );
    }

    #[test]
    fn provider_error_is_reported_as_filesystem_error() {
        let directory = TestDirectory::new("provider_error");
        let path = directory.path();

        let result = filesystem_scope::resolve(path, provider_error);

        assert!(matches!(result, Err(ScopeError::Filesystem(_))));
    }

    #[test]
    fn scope_setter_set_matches_set_filesystem_scope_use_case() {
        let directory = TestDirectory::new("set_use_case_pointer");
        let set_scope: SetFilesystemScope = scope_setter::set;

        let scope = set_scope(directory.path()).expect("agent should match the use case signature");

        assert_eq!(
            scope.path(),
            directory.path().canonicalize().unwrap().as_path()
        );
    }

    #[test]
    fn scope_setter_returns_resolved_path_for_normal_path() {
        let directory = TestDirectory::new("set_resolved_normal");

        let scope = scope_setter::set(directory.path()).unwrap();

        assert_eq!(
            scope.path(),
            directory.path().canonicalize().unwrap().as_path()
        );
    }

    #[test]
    fn scope_setter_resolves_child_parent_without_preserving_parent_component() {
        let directory = TestDirectory::new("set_resolved_parent_component");
        let child = directory.path().join("child");
        fs::create_dir(&child).unwrap();

        let scope = scope_setter::set(child.join("..").as_path()).unwrap();

        assert_eq!(
            scope.path(),
            directory.path().canonicalize().unwrap().as_path()
        );
        assert!(
            !scope
                .path()
                .components()
                .any(|component| { matches!(component, std::path::Component::ParentDir) })
        );
    }

    #[test]
    fn scope_setter_resolves_current_directory_component() {
        let directory = TestDirectory::new("set_resolved_current_component");

        let scope = scope_setter::set(directory.path().join(".").as_path()).unwrap();

        assert_eq!(
            scope.path(),
            directory.path().canonicalize().unwrap().as_path()
        );
    }

    #[test]
    fn iter_accepts_valid_filesystem_scope_and_returns_iteration() {
        let directory = TestDirectory::new("iter_accepts_scope");
        let scope = scope_setter::set(directory.path()).unwrap();

        let result = iterator::iter(&scope);

        assert!(result.is_ok());
    }

    #[test]
    fn iterator_iter_matches_iter_use_case_function_pointer() {
        let directory = TestDirectory::new("iter_use_case_pointer");
        let scope = scope_setter::set(directory.path()).unwrap();
        let iter_scope: Iter = iterator::iter;

        let result = iter_scope(&scope);

        assert!(result.is_ok());
    }

    #[test]
    fn iteration_advancer_advance_matches_advance_use_case_function_pointer() {
        let directory = TestDirectory::new("advance_use_case_pointer");
        let scope = scope_setter::set(directory.path()).unwrap();
        let mut iteration = iterator::iter(&scope).unwrap();
        let advance: Advance = iteration_advancer::advance;

        let result = advance(&mut iteration);

        assert!(matches!(result, Ok(None)));
    }

    #[test]
    fn filesystem_iteration_starts_with_next_index_zero() {
        let directory = TestDirectory::new("iteration_index_zero");
        let scope = scope_setter::set(directory.path()).unwrap();
        let iteration = iterator::iter(&scope).unwrap();

        assert_eq!(iteration.next_index(), 0);
    }

    #[test]
    fn filesystem_iteration_preserves_scope_path() {
        let directory = TestDirectory::new("iteration_path");
        let scope = scope_setter::set(directory.path()).unwrap();
        let iteration = iterator::iter(&scope).unwrap();

        assert_eq!(iteration.path(), scope.path());
    }

    #[test]
    fn creating_filesystem_iteration_does_not_modify_scope() {
        let directory = TestDirectory::new("iteration_scope_unchanged");
        let scope = scope_setter::set(directory.path()).unwrap();
        let scope_path = scope.path().to_path_buf();

        let iteration = iterator::iter(&scope).unwrap();

        assert_eq!(scope.path(), scope_path.as_path());
        assert_eq!(iteration.path(), scope_path.as_path());
    }

    #[test]
    fn empty_filesystem_iteration_preserves_path_after_none() {
        let directory = TestDirectory::new("empty_iteration_path");
        let scope = scope_setter::set(directory.path()).unwrap();
        let mut iteration = iterator::iter(&scope).unwrap();
        let path = iteration.path().to_path_buf();

        let result = iteration_advancer::advance(&mut iteration).unwrap();

        assert!(result.is_none());
        assert_eq!(iteration.path(), path.as_path());
        assert_eq!(iteration.next_index(), 0);
    }

    #[test]
    fn enterer_enter_matches_enter_use_case_function_pointer() {
        let directory = TestDirectory::new("enter_use_case_pointer");
        let child = directory.path().join("child");
        fs::create_dir(&child).unwrap();
        let scope = scope_setter::set(directory.path()).unwrap();
        let enter: Enter = enterer::enter;

        let result = enter(&scope, Path::new("child")).unwrap();

        assert_eq!(result.path(), child.as_path());
    }

    #[test]
    fn enter_child_returns_new_filesystem_scope_for_child() {
        let directory = TestDirectory::new("enter_child");
        let child = directory.path().join("child");
        fs::create_dir(&child).unwrap();
        let scope = scope_setter::set(directory.path()).unwrap();

        let result = enterer::enter(&scope, Path::new("child")).unwrap();

        assert_eq!(result.path(), child.as_path());
    }

    #[test]
    fn enter_compound_child_path_returns_new_filesystem_scope() {
        let directory = TestDirectory::new("enter_compound_child");
        let child = directory.path().join("child");
        let grandchild = child.join("grandchild");
        fs::create_dir(&child).unwrap();
        fs::create_dir(&grandchild).unwrap();
        let scope = scope_setter::set(directory.path()).unwrap();

        let result = enterer::enter(&scope, Path::new("child/grandchild")).unwrap();

        assert_eq!(result.path(), grandchild.as_path());
    }

    #[test]
    fn enter_parent_from_child_returns_parent_scope() {
        let directory = TestDirectory::new("enter_parent");
        let child = directory.path().join("child");
        fs::create_dir(&child).unwrap();
        let scope = scope_setter::set(child.as_path()).unwrap();

        let result = enterer::enter(&scope, Path::new("..")).unwrap();

        assert_eq!(
            result.path(),
            directory.path().canonicalize().unwrap().as_path()
        );
        assert!(
            !result
                .path()
                .components()
                .any(|component| { matches!(component, std::path::Component::ParentDir) })
        );
    }

    #[test]
    fn enter_two_parents_from_deep_scope_returns_ancestor_scope() {
        let directory = TestDirectory::new("enter_two_parents");
        let child = directory.path().join("child");
        let grandchild = child.join("grandchild");
        fs::create_dir(&child).unwrap();
        fs::create_dir(&grandchild).unwrap();
        let scope = scope_setter::set(grandchild.as_path()).unwrap();

        let result = enterer::enter(&scope, Path::new("../..")).unwrap();

        assert_eq!(
            result.path(),
            directory.path().canonicalize().unwrap().as_path()
        );
        assert!(
            !result
                .path()
                .components()
                .any(|component| { matches!(component, std::path::Component::ParentDir) })
        );
    }

    #[test]
    fn enter_nonexistent_location_returns_scope_error() {
        let directory = TestDirectory::new("enter_missing");
        let scope = scope_setter::set(directory.path()).unwrap();

        let result = enterer::enter(&scope, Path::new("missing"));

        assert!(matches!(result, Err(ScopeError::Filesystem(_))));
    }

    #[test]
    fn enter_does_not_modify_original_filesystem_scope() {
        let directory = TestDirectory::new("enter_original_unchanged");
        let child = directory.path().join("child");
        fs::create_dir(&child).unwrap();
        let scope = scope_setter::set(directory.path()).unwrap();

        let result = enterer::enter(&scope, Path::new("child")).unwrap();

        assert_eq!(scope.path(), directory.path());
        assert_eq!(result.path(), child.as_path());
    }

    #[test]
    fn filesystem_path_resolve_only_produces_candidate_path() {
        let directory = TestDirectory::new("path_resolve_only");
        let scope = scope_setter::set(directory.path()).unwrap();

        let result = filesystem_path::resolve(&scope, Path::new("missing"));

        assert_eq!(result, directory.path().join("missing"));
    }

    #[test]
    fn filesystem_path_resolve_can_preserve_parent_component_in_candidate() {
        let directory = TestDirectory::new("path_resolve_parent_component");
        let child = directory.path().join("child");
        fs::create_dir(&child).unwrap();
        let scope = scope_setter::set(child.as_path()).unwrap();

        let result = filesystem_path::resolve(&scope, Path::new(".."));

        assert_eq!(result, child.join(".."));
        assert!(
            result
                .components()
                .any(|component| { matches!(component, std::path::Component::ParentDir) })
        );
    }

    #[cfg(unix)]
    #[test]
    fn scope_setter_resolves_symlink_using_filesystem_semantics() {
        use std::os::unix::fs::symlink;

        let directory = TestDirectory::new("set_resolved_symlink");
        let target = directory.path().join("target");
        let link = directory.path().join("link");
        fs::create_dir(&target).unwrap();
        symlink(&target, &link).unwrap();

        let scope = scope_setter::set(link.as_path()).unwrap();

        assert_eq!(scope.path(), target.canonicalize().unwrap().as_path());
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
        let scope = scope_setter::set(directory.path()).unwrap();
        let mut iteration = iterator::iter(&scope).unwrap();
        let next_directory_entry: NextDirectoryEntry = providers::next_directory_entry::provide;

        let result = next_directory_entry(&mut iteration);

        assert!(matches!(result, Ok(None)));
    }

    #[test]
    fn filesystem_entry_resolve_returns_one_entry_per_call_and_reaches_end() {
        let directory = TestDirectory::new("entry_one_per_call");
        fs::write(directory.path().join("report.txt"), "report").unwrap();
        fs::create_dir(directory.path().join("images")).unwrap();
        let scope = scope_setter::set(directory.path()).unwrap();
        let mut iteration = iterator::iter(&scope).unwrap();

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
    fn iteration_advancer_advance_returns_at_most_one_entry_per_call() {
        let directory = TestDirectory::new("advance_one_entry");
        fs::write(directory.path().join("report.txt"), "report").unwrap();
        let scope = scope_setter::set(directory.path()).unwrap();
        let mut iteration = iterator::iter(&scope).unwrap();

        let first = iteration_advancer::advance(&mut iteration).unwrap();
        let end = iteration_advancer::advance(&mut iteration).unwrap();

        assert!(first.is_some());
        assert!(end.is_none());
    }

    #[test]
    fn iteration_advancer_advance_produces_entries_one_by_one_until_none() {
        let directory = TestDirectory::new("advance_until_none");
        fs::write(directory.path().join("report.txt"), "report").unwrap();
        fs::create_dir(directory.path().join("images")).unwrap();
        let scope = scope_setter::set(directory.path()).unwrap();
        let mut iteration = iterator::iter(&scope).unwrap();

        let first = iteration_advancer::advance(&mut iteration).unwrap();
        let second = iteration_advancer::advance(&mut iteration).unwrap();
        let end = iteration_advancer::advance(&mut iteration).unwrap();

        assert!(first.is_some());
        assert!(second.is_some());
        assert!(end.is_none());
    }

    #[test]
    fn iteration_advancer_assigns_incremental_indexes_to_produced_items() {
        let directory = TestDirectory::new("advance_indexes");
        fs::write(directory.path().join("first.txt"), "first").unwrap();
        fs::write(directory.path().join("second.txt"), "second").unwrap();
        let scope = scope_setter::set(directory.path()).unwrap();
        let mut iteration = iterator::iter(&scope).unwrap();

        let first = iteration_advancer::advance(&mut iteration)
            .unwrap()
            .expect("first item should exist");
        let second = iteration_advancer::advance(&mut iteration)
            .unwrap()
            .expect("second item should exist");
        let end = iteration_advancer::advance(&mut iteration).unwrap();

        assert_eq!(first.index(), 0);
        assert_eq!(second.index(), 1);
        assert!(
            first.entry().name() == OsStr::new("first.txt")
                || first.entry().name() == OsStr::new("second.txt")
        );
        assert!(
            second.entry().name() == OsStr::new("first.txt")
                || second.entry().name() == OsStr::new("second.txt")
        );
        assert!(end.is_none());
        assert_eq!(iteration.next_index(), 2);
    }

    #[test]
    fn filesystem_iteration_item_exposes_index_separately_from_entry() {
        let directory = TestDirectory::new("iteration_item_entity");
        fs::write(directory.path().join("report.txt"), "report").unwrap();
        let scope = scope_setter::set(directory.path()).unwrap();
        let mut iteration = iterator::iter(&scope).unwrap();

        let item: FilesystemIterationItem = iteration_advancer::advance(&mut iteration)
            .unwrap()
            .expect("item should exist");

        assert_eq!(item.index(), 0);
        assert_eq!(item.entry().name(), OsStr::new("report.txt"));
        assert_eq!(item.entry().kind(), FilesystemEntryKind::File);
    }

    #[test]
    fn public_advance_distinguishes_file_and_directory() {
        let directory = TestDirectory::new("public_advance_kind");
        fs::write(directory.path().join("report.txt"), "report").unwrap();
        fs::create_dir(directory.path().join("images")).unwrap();
        let scope = scope_setter::set(directory.path()).unwrap();
        let mut iteration = iterator::iter(&scope).unwrap();
        let advance: Advance = iteration_advancer::advance;
        let mut found_file = false;
        let mut found_directory = false;

        while let Some(item) = advance(&mut iteration).unwrap() {
            let entry = item.entry();
            if entry.name() == OsStr::new("report.txt") {
                assert_eq!(entry.kind(), FilesystemEntryKind::File);
                found_file = true;
            }

            if entry.name() == OsStr::new("images") {
                assert_eq!(entry.kind(), FilesystemEntryKind::Directory);
                found_directory = true;
            }
        }

        assert!(found_file);
        assert!(found_directory);
    }

    #[test]
    fn filesystem_entry_resolve_distinguishes_file_and_directory() {
        let directory = TestDirectory::new("entry_kind");
        fs::write(directory.path().join("report.txt"), "report").unwrap();
        fs::create_dir(directory.path().join("images")).unwrap();
        let scope = scope_setter::set(directory.path()).unwrap();
        let mut iteration = iterator::iter(&scope).unwrap();
        let mut found_file = false;
        let mut found_directory = false;

        while let Some(entry) =
            filesystem_entry::resolve(&mut iteration, providers::next_directory_entry::provide)
                .unwrap()
        {
            if entry.name() == OsStr::new("report.txt") {
                let path = directory.path().join("report.txt");
                let metadata = fs::metadata(&path).unwrap();

                assert_eq!(entry.path(), path.as_path());
                assert_eq!(entry.kind(), FilesystemEntryKind::File);
                assert_eq!(entry.size(), Some(6));
                assert_eq!(entry.created(), metadata.created().ok());
                assert!(entry.modified().is_some());
                found_file = true;
            }

            if entry.name() == OsStr::new("images") {
                let path = directory.path().join("images");
                let metadata = fs::metadata(&path).unwrap();

                assert_eq!(entry.path(), path.as_path());
                assert_eq!(entry.kind(), FilesystemEntryKind::Directory);
                assert_eq!(entry.size(), None);
                assert_eq!(entry.created(), metadata.created().ok());
                assert!(entry.modified().is_some());
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
        let scope = scope_setter::set(directory.path()).unwrap();
        let mut iteration = iterator::iter(&scope).unwrap();
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
    fn filesystem_entry_resolve_reports_file_size_for_empty_file() {
        let directory = TestDirectory::new("entry_empty_file_size");
        fs::write(directory.path().join("empty.txt"), "").unwrap();
        let scope = scope_setter::set(directory.path()).unwrap();
        let mut iteration = iterator::iter(&scope).unwrap();

        let entry =
            filesystem_entry::resolve(&mut iteration, providers::next_directory_entry::provide)
                .unwrap()
                .expect("empty file should exist");

        assert_eq!(entry.name(), OsStr::new("empty.txt"));
        assert_eq!(entry.kind(), FilesystemEntryKind::File);
        assert_eq!(entry.size(), Some(0));
        assert_eq!(
            entry.created(),
            fs::metadata(directory.path().join("empty.txt"))
                .unwrap()
                .created()
                .ok()
        );
    }

    #[test]
    fn filesystem_entry_resolve_reports_metadata_error() {
        let directory = TestDirectory::new("entry_metadata_error");
        let file = directory.path().join("removed.txt");
        fs::write(&file, "removed").unwrap();
        let dir_entry = fs::read_dir(directory.path())
            .unwrap()
            .next()
            .unwrap()
            .unwrap();
        fs::remove_file(&file).unwrap();
        *REMOVED_DIRECTORY_ENTRY.lock().unwrap() = Some(dir_entry);
        let scope = scope_setter::set(directory.path()).unwrap();
        let mut iteration = iterator::iter(&scope).unwrap();

        let result = filesystem_entry::resolve(&mut iteration, removed_directory_entry);

        assert!(matches!(result, Err(IterError::MaterializeEntry(_))));
    }

    #[cfg(unix)]
    #[test]
    fn public_advance_distinguishes_symlink() {
        use std::os::unix::fs::symlink;

        let directory = TestDirectory::new("public_advance_symlink");
        fs::write(directory.path().join("report.txt"), "report").unwrap();
        symlink(
            directory.path().join("report.txt"),
            directory.path().join("report-link.txt"),
        )
        .unwrap();
        let scope = scope_setter::set(directory.path()).unwrap();
        let mut iteration = iterator::iter(&scope).unwrap();
        let mut found_symlink = false;

        while let Some(item) = iteration_advancer::advance(&mut iteration).unwrap() {
            let entry = item.entry();
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
        let scope = filesystem_scope::resolve(directory.path(), always_directory).unwrap();

        let result = filesystem_iteration::resolve(&scope, read_directory_error);

        assert!(matches!(result, Err(IterError::OpenDirectory(_))));
    }

    #[test]
    fn iterator_iter_reports_public_open_directory_error_without_external_provider() {
        let directory = TestDirectory::new("public_open_error");
        let scope = scope_setter::set(directory.path()).unwrap();
        fs::remove_dir_all(directory.path()).unwrap();

        let result = iterator::iter(&scope);

        assert!(matches!(result, Err(IterError::OpenDirectory(_))));
    }

    #[test]
    fn filesystem_entry_resolve_reports_next_entry_error() {
        let directory = TestDirectory::new("next_error");
        let scope = scope_setter::set(directory.path()).unwrap();
        let mut iteration = iterator::iter(&scope).unwrap();

        let result = filesystem_entry::resolve(&mut iteration, next_directory_entry_error);

        assert!(matches!(result, Err(IterError::NextEntry(_))));
    }
}
