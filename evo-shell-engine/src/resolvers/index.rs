use crate::definitions::domain::value_objects::structured_items::StructuredItems;
use crate::definitions::use_cases::index::IndexError;

pub fn resolve<'a>(
    items: StructuredItems<'a>,
    index: usize,
) -> Result<StructuredItems<'a>, IndexError> {
    let mut matches = items.iter().filter(|item| item.index() == index);

    let found = matches.next().ok_or_else(|| IndexError::not_found(index))?;

    if matches.next().is_some() {
        return Err(IndexError::ambiguous_index(index));
    }

    Ok(StructuredItems::single(found))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::agents::indexer;
    use crate::definitions::domain::entities::filesystem_entry::{
        FilesystemEntry, FilesystemEntryKind,
    };
    use crate::definitions::domain::entities::filesystem_iteration_item::FilesystemIterationItem;
    use crate::definitions::domain::value_objects::select::SelectProperty;
    use crate::definitions::domain::value_objects::structured_items::StructuredItems;
    use crate::definitions::use_cases::index::Index;
    use std::ffi::OsString;
    use std::path::PathBuf;
    use std::time::{Duration, SystemTime};

    fn time(seconds: u64) -> SystemTime {
        SystemTime::UNIX_EPOCH + Duration::from_secs(seconds)
    }

    fn entry(
        name: &str,
        kind: FilesystemEntryKind,
        created: Option<SystemTime>,
        modified: Option<SystemTime>,
        size: Option<u64>,
    ) -> FilesystemEntry {
        FilesystemEntry::new(
            OsString::from(name),
            PathBuf::from(format!("/tmp/{name}")),
            kind,
            created,
            modified,
            size,
        )
    }

    fn item(
        index: usize,
        name: &str,
        kind: FilesystemEntryKind,
        created: Option<SystemTime>,
        modified: Option<SystemTime>,
        size: Option<u64>,
    ) -> FilesystemIterationItem {
        FilesystemIterationItem::new(index, entry(name, kind, created, modified, size))
    }

    fn sample_items() -> Vec<FilesystemIterationItem> {
        vec![
            item(
                0,
                "README.md",
                FilesystemEntryKind::File,
                Some(time(10)),
                Some(time(20)),
                Some(10),
            ),
            item(
                1,
                "src",
                FilesystemEntryKind::Directory,
                Some(time(30)),
                Some(time(40)),
                None,
            ),
            item(
                2,
                "notes.txt",
                FilesystemEntryKind::File,
                Some(time(50)),
                Some(time(60)),
                Some(30),
            ),
        ]
    }

    fn duplicate_index_items() -> Vec<FilesystemIterationItem> {
        vec![
            item(
                1,
                "first",
                FilesystemEntryKind::File,
                Some(time(10)),
                Some(time(20)),
                Some(10),
            ),
            item(
                1,
                "second",
                FilesystemEntryKind::Directory,
                Some(time(30)),
                Some(time(40)),
                Some(20),
            ),
        ]
    }

    fn structured_items<'a>(items: &'a [FilesystemIterationItem]) -> StructuredItems<'a> {
        StructuredItems::from_slice(items)
    }

    fn empty_items<'a>() -> StructuredItems<'a> {
        StructuredItems::new(Vec::new())
    }

    #[test]
    fn indexer_matches_use_case_function_pointer() {
        let index_case: Index = indexer::index;
        let items = sample_items();

        let result = index_case(structured_items(&items), 1).unwrap();

        assert_eq!(result.len(), 1);
        assert!(std::ptr::eq(result.items()[0], &items[1]));
    }

    #[test]
    fn index_returns_exact_match_for_first_item() {
        let items = sample_items();

        let result = resolve(structured_items(&items), 0).unwrap();

        assert_eq!(result.len(), 1);
        assert!(std::ptr::eq(result.items()[0], &items[0]));
    }

    #[test]
    fn index_returns_exact_match_for_middle_item() {
        let items = sample_items();

        let result = resolve(structured_items(&items), 1).unwrap();

        assert_eq!(result.len(), 1);
        assert!(std::ptr::eq(result.items()[0], &items[1]));
    }

    #[test]
    fn index_returns_exact_match_for_last_item() {
        let items = sample_items();

        let result = resolve(structured_items(&items), 2).unwrap();

        assert_eq!(result.len(), 1);
        assert!(std::ptr::eq(result.items()[0], &items[2]));
    }

    #[test]
    fn index_returns_not_found_for_empty_collection() {
        let result = resolve(empty_items(), 0);

        assert!(matches!(
            result,
            Err(IndexError::NotFound { index }) if index == 0
        ));
    }

    #[test]
    fn index_returns_not_found_when_missing() {
        let items = sample_items();

        let result = resolve(structured_items(&items), 99);

        assert!(matches!(
            result,
            Err(IndexError::NotFound { index }) if index == 99
        ));
    }

    #[test]
    fn index_detects_ambiguous_indices() {
        let items = duplicate_index_items();

        let result = resolve(structured_items(&items), 1);

        assert!(matches!(
            result,
            Err(IndexError::AmbiguousIndex { index }) if index == 1
        ));
    }

    #[test]
    fn index_preserves_complete_item_and_original_index() {
        let items = sample_items();

        let result = resolve(structured_items(&items), 2).unwrap();
        let indexed = result.items()[0];

        assert_eq!(indexed.index(), items[2].index());
        assert!(std::ptr::eq(indexed, &items[2]));
        assert_eq!(indexed.entry().name(), items[2].entry().name());
    }

    #[test]
    fn index_result_can_feed_select_without_reconstruction() {
        let items = sample_items();
        let indexed = resolve(structured_items(&items), 1).unwrap();
        let projection =
            crate::resolvers::select::resolve(indexed, &[SelectProperty::Name]).unwrap();

        assert_eq!(projection.rows().len(), 1);
        assert_eq!(projection.rows()[0].values().len(), 1);
    }
}
