use evo_shell::definitions::structs::filesystem_field::{FILESYSTEM_FIELDS, FilesystemField};

#[test]
fn filesystem_fields_contains_exact_elements_and_order() {
    assert_eq!(FILESYSTEM_FIELDS.len(), 5);
    assert_eq!(
        FILESYSTEM_FIELDS,
        &[
            FilesystemField::Index,
            FilesystemField::Name,
            FilesystemField::Path,
            FilesystemField::Kind,
            FilesystemField::Size,
        ]
    );
}
