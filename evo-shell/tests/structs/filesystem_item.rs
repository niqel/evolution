use evo_shell::definitions::structs::borrowed::filesystem_item::FilesystemItem;
use evo_shell::definitions::structs::filesystem_item_kind::FilesystemItemKind;

#[test]
fn filesystem_item_creation_and_field_preservation() {
    let name = "report.md";
    let path = "/home/user/documents/report.md";

    let item = FilesystemItem {
        index: 0,
        name,
        path,
        kind: FilesystemItemKind::File,
        size: Some(128),
    };

    assert_eq!(item.index, 0);
    assert_eq!(item.name, name);
    assert_eq!(item.path, path);
    assert_eq!(item.kind, FilesystemItemKind::File);
    assert_eq!(item.size, Some(128));
}

#[test]
fn filesystem_item_directory_without_size() {
    let name = "documents";
    let path = "/home/user/documents";

    let item = FilesystemItem {
        index: 1,
        name,
        path,
        kind: FilesystemItemKind::Directory,
        size: None,
    };

    assert_eq!(item.index, 1);
    assert_eq!(item.name, name);
    assert_eq!(item.path, path);
    assert_eq!(item.kind, FilesystemItemKind::Directory);
    assert_eq!(item.size, None);
}
