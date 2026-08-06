use crate::definitions::domain::entities::filesystem_iteration_item::FilesystemIterationItem;
use crate::definitions::domain::value_objects::select::{
    ProjectedRow, ProjectedValue, SelectProperty, StructuredProjection,
};
use crate::definitions::use_cases::select::SelectError;

pub fn resolve(
    items: &[FilesystemIterationItem],
    properties: &[SelectProperty],
) -> Result<StructuredProjection, SelectError> {
    let selected_properties = validate_properties(properties)?;

    let mut rows = Vec::with_capacity(items.len());

    for item in items {
        rows.push(ProjectedRow::new(project_row(item, &selected_properties)));
    }

    Ok(StructuredProjection::new(selected_properties, rows))
}

fn validate_properties(properties: &[SelectProperty]) -> Result<Vec<SelectProperty>, SelectError> {
    let mut selected = Vec::with_capacity(properties.len());

    for property in properties {
        match property {
            SelectProperty::Unsupported(name) => {
                return Err(SelectError::UnsupportedProperty(name.clone()));
            }
            _ => selected.push(property.clone()),
        }
    }

    Ok(selected)
}

fn project_row(
    item: &FilesystemIterationItem,
    properties: &[SelectProperty],
) -> Vec<ProjectedValue> {
    let mut values = Vec::with_capacity(properties.len());

    for property in properties {
        values.push(project_value(item, property));
    }

    values
}

fn project_value(item: &FilesystemIterationItem, property: &SelectProperty) -> ProjectedValue {
    match property {
        SelectProperty::Index => ProjectedValue::index(item.index()),
        SelectProperty::Created => ProjectedValue::created(item.entry().created()),
        SelectProperty::Modified => ProjectedValue::modified(item.entry().modified()),
        SelectProperty::Type => ProjectedValue::kind(item.entry().kind()),
        SelectProperty::Size => ProjectedValue::size(item.entry().size()),
        SelectProperty::Name => ProjectedValue::name(item.entry().name().to_os_string()),
        SelectProperty::Unsupported(_) => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::agents::selector;
    use crate::definitions::domain::entities::filesystem_entry::{
        FilesystemEntry, FilesystemEntryKind,
    };
    use crate::definitions::domain::entities::filesystem_iteration_item::FilesystemIterationItem;
    use crate::definitions::domain::value_objects::select::{ProjectedValue, SelectProperty};
    use crate::definitions::use_cases::select::Select;
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
                None,
                None,
                Some(30),
            ),
        ]
    }

    fn select_name() -> Vec<SelectProperty> {
        vec![SelectProperty::Name]
    }

    fn select_index() -> Vec<SelectProperty> {
        vec![SelectProperty::Index]
    }

    fn select_type() -> Vec<SelectProperty> {
        vec![SelectProperty::Type]
    }

    fn select_size() -> Vec<SelectProperty> {
        vec![SelectProperty::Size]
    }

    fn select_created() -> Vec<SelectProperty> {
        vec![SelectProperty::Created]
    }

    fn select_modified() -> Vec<SelectProperty> {
        vec![SelectProperty::Modified]
    }

    fn select_name_size() -> Vec<SelectProperty> {
        vec![SelectProperty::Name, SelectProperty::Size]
    }

    fn select_size_name() -> Vec<SelectProperty> {
        vec![SelectProperty::Size, SelectProperty::Name]
    }

    fn select_multiple() -> Vec<SelectProperty> {
        vec![
            SelectProperty::Index,
            SelectProperty::Created,
            SelectProperty::Modified,
            SelectProperty::Type,
            SelectProperty::Size,
            SelectProperty::Name,
        ]
    }

    #[test]
    fn selector_matches_use_case_function_pointer() {
        let select_case: Select = selector::select;
        let items = sample_items();

        let projection = select_case(&items, &select_name()).unwrap();

        assert_eq!(projection.rows().len(), 3);
    }

    #[test]
    fn empty_input_returns_empty_projection() {
        let projection = resolve(&[], &select_name()).unwrap();

        assert!(projection.rows().is_empty());
        assert_eq!(projection.properties(), &[SelectProperty::Name]);
    }

    #[test]
    fn select_name_projects_single_property_without_strings_everywhere() {
        let items = sample_items();
        let projection = resolve(&items, &select_name()).unwrap();

        assert_eq!(projection.properties(), &[SelectProperty::Name]);
        assert_eq!(projection.rows().len(), 3);
        assert!(matches!(
            projection.rows()[0].values()[0],
            ProjectedValue::Name(_)
        ));
        assert!(matches!(
            projection.rows()[1].values()[0],
            ProjectedValue::Name(_)
        ));
        assert!(matches!(
            projection.rows()[2].values()[0],
            ProjectedValue::Name(_)
        ));
    }

    #[test]
    fn select_index_projects_existing_indices() {
        let items = sample_items();
        let projection = resolve(&items, &select_index()).unwrap();

        assert_eq!(projection.rows().len(), 3);
        assert_eq!(projection.rows()[0].values()[0], ProjectedValue::Index(0));
        assert_eq!(projection.rows()[1].values()[0], ProjectedValue::Index(1));
        assert_eq!(projection.rows()[2].values()[0], ProjectedValue::Index(2));
    }

    #[test]
    fn select_type_projects_entry_kind() {
        let items = sample_items();
        let projection = resolve(&items, &select_type()).unwrap();

        assert_eq!(
            projection.rows()[0].values()[0],
            ProjectedValue::Type(FilesystemEntryKind::File)
        );
        assert_eq!(
            projection.rows()[1].values()[0],
            ProjectedValue::Type(FilesystemEntryKind::Directory)
        );
    }

    #[test]
    fn select_size_preserves_optional_absence_without_error() {
        let items = sample_items();
        let projection = resolve(&items, &select_size()).unwrap();

        assert_eq!(projection.rows().len(), 3);
        assert_eq!(
            projection.rows()[0].values()[0],
            ProjectedValue::Size(Some(10))
        );
        assert_eq!(projection.rows()[1].values()[0], ProjectedValue::Size(None));
        assert_eq!(
            projection.rows()[2].values()[0],
            ProjectedValue::Size(Some(30))
        );
    }

    #[test]
    fn select_created_preserves_optional_absence_without_error() {
        let items = sample_items();
        let projection = resolve(&items, &select_created()).unwrap();

        assert_eq!(
            projection.rows()[2].values()[0],
            ProjectedValue::Created(None)
        );
    }

    #[test]
    fn select_modified_preserves_optional_absence_without_error() {
        let items = sample_items();
        let projection = resolve(&items, &select_modified()).unwrap();

        assert_eq!(
            projection.rows()[2].values()[0],
            ProjectedValue::Modified(None)
        );
    }

    #[test]
    fn select_name_size_preserves_property_order() {
        let items = sample_items();
        let projection = resolve(&items, &select_name_size()).unwrap();

        assert_eq!(
            projection.properties(),
            &[SelectProperty::Name, SelectProperty::Size]
        );
        assert_eq!(
            projection.rows()[0].values(),
            &[
                ProjectedValue::Name(OsString::from("README.md")),
                ProjectedValue::Size(Some(10))
            ]
        );
    }

    #[test]
    fn select_size_name_preserves_requested_order() {
        let items = sample_items();
        let projection = resolve(&items, &select_size_name()).unwrap();

        assert_eq!(
            projection.properties(),
            &[SelectProperty::Size, SelectProperty::Name]
        );
        assert_eq!(
            projection.rows()[0].values(),
            &[
                ProjectedValue::Size(Some(10)),
                ProjectedValue::Name(OsString::from("README.md"))
            ]
        );
    }

    #[test]
    fn select_preserves_row_order() {
        let items = sample_items();
        let projection = resolve(&items, &select_name()).unwrap();

        assert_eq!(
            projection.rows()[0].values()[0],
            ProjectedValue::Name(OsString::from("README.md"))
        );
        assert_eq!(
            projection.rows()[1].values()[0],
            ProjectedValue::Name(OsString::from("src"))
        );
        assert_eq!(
            projection.rows()[2].values()[0],
            ProjectedValue::Name(OsString::from("notes.txt"))
        );
    }

    #[test]
    fn unsupported_property_returns_error() {
        let items = sample_items();
        let properties = vec![SelectProperty::unsupported("mystery")];

        let result = resolve(&items, &properties);

        assert!(matches!(
            result,
            Err(SelectError::UnsupportedProperty(property)) if property == "mystery"
        ));
    }

    #[test]
    fn select_multiple_properties_preserves_types_without_stringifying() {
        let items = sample_items();
        let projection = resolve(&items, &select_multiple()).unwrap();

        assert_eq!(projection.properties().len(), 6);
        assert!(matches!(
            projection.rows()[0].values()[0],
            ProjectedValue::Index(_)
        ));
        assert!(matches!(
            projection.rows()[0].values()[1],
            ProjectedValue::Created(Some(_))
        ));
        assert!(matches!(
            projection.rows()[0].values()[2],
            ProjectedValue::Modified(Some(_))
        ));
        assert!(matches!(
            projection.rows()[0].values()[3],
            ProjectedValue::Type(_)
        ));
        assert!(matches!(
            projection.rows()[0].values()[4],
            ProjectedValue::Size(Some(_))
        ));
        assert!(matches!(
            projection.rows()[0].values()[5],
            ProjectedValue::Name(_)
        ));
    }
}
