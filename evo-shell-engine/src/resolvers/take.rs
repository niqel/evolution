use crate::definitions::domain::value_objects::structured_items::StructuredItems;

pub fn resolve<'a>(items: StructuredItems<'a>, count: usize) -> StructuredItems<'a> {
    StructuredItems::new(items.items().iter().copied().take(count).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::agents::taker;
    use crate::definitions::domain::entities::filesystem_entry::{
        FilesystemEntry, FilesystemEntryKind,
    };
    use crate::definitions::domain::entities::filesystem_iteration_item::FilesystemIterationItem;
    use crate::definitions::domain::value_objects::select::SelectProperty;
    use crate::definitions::domain::value_objects::structured_items::StructuredItems;
    use crate::definitions::use_cases::take::Take;
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
            item(
                3,
                "temp.txt",
                FilesystemEntryKind::File,
                None,
                None,
                Some(5),
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
    fn taker_matches_use_case_function_pointer() {
        let take_case: Take = taker::take;
        let items = sample_items();

        let result = take_case(structured_items(&items), 2);

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn take_empty_collection_returns_empty_collection() {
        let result = resolve(empty_items(), 1);

        assert!(result.is_empty());
    }

    #[test]
    fn take_zero_returns_empty_collection() {
        let items = sample_items();

        let result = resolve(structured_items(&items), 0);

        assert!(result.is_empty());
    }

    #[test]
    fn take_one_returns_first_item_only() {
        let items = sample_items();

        let result = resolve(structured_items(&items), 1);

        assert_eq!(result.len(), 1);
        assert!(std::ptr::eq(result.items()[0], &items[0]));
    }

    #[test]
    fn take_equal_to_length_keeps_all_items() {
        let items = sample_items();

        let result = resolve(structured_items(&items), items.len());

        assert_eq!(result.len(), items.len());
        assert!(std::ptr::eq(result.items()[0], &items[0]));
        assert!(std::ptr::eq(result.items()[1], &items[1]));
        assert!(std::ptr::eq(result.items()[2], &items[2]));
        assert!(std::ptr::eq(result.items()[3], &items[3]));
    }

    #[test]
    fn take_less_than_length_limits_collection() {
        let items = sample_items();

        let result = resolve(structured_items(&items), 2);

        assert_eq!(result.len(), 2);
        assert!(std::ptr::eq(result.items()[0], &items[0]));
        assert!(std::ptr::eq(result.items()[1], &items[1]));
    }

    #[test]
    fn take_greater_than_length_keeps_all_items() {
        let items = sample_items();

        let result = resolve(structured_items(&items), 10);

        assert_eq!(result.len(), items.len());
        assert!(std::ptr::eq(result.items()[0], &items[0]));
        assert!(std::ptr::eq(result.items()[1], &items[1]));
        assert!(std::ptr::eq(result.items()[2], &items[2]));
        assert!(std::ptr::eq(result.items()[3], &items[3]));
    }

    #[test]
    fn take_preserves_order_and_identity() {
        let items = sample_items();

        let result = resolve(structured_items(&items), 3);

        assert_eq!(result.len(), 3);
        assert!(std::ptr::eq(result.items()[0], &items[0]));
        assert!(std::ptr::eq(result.items()[1], &items[1]));
        assert!(std::ptr::eq(result.items()[2], &items[2]));
    }

    #[test]
    fn take_preserves_original_indices_without_reindexing() {
        let items = sample_items();

        let result = resolve(structured_items(&items), 2);

        assert_eq!(result.items()[0].index(), 0);
        assert_eq!(result.items()[1].index(), 1);
    }

    #[test]
    fn take_result_can_feed_select_without_reconstruction() {
        let items = sample_items();
        let taken = resolve(structured_items(&items), 2);
        let projection = crate::resolvers::select::resolve(taken, &[SelectProperty::Name]).unwrap();

        assert_eq!(projection.rows().len(), 2);
    }
}
