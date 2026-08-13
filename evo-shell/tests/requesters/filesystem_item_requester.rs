use evo_shell::definitions::requesters::filesystem_item_requester;
use evo_shell::definitions::structs::borrowed::filesystem_item::FilesystemItem;
use evo_shell::definitions::structs::filesystem_item_kind::FilesystemItemKind;

fn receive_item(item: FilesystemItem<'_>) {
    assert_eq!(item.index, 7);
    assert_eq!(item.name, "report.md");
    assert_eq!(item.path, "/home/user/documents/report.md");
    assert_eq!(item.kind, FilesystemItemKind::File);
    assert_eq!(item.size, Some(512));
}

#[test]
fn filesystem_item_requester_accepts_borrowed_item() {
    let request: filesystem_item_requester::Request = receive_item;

    let name = "report.md";
    let path = "/home/user/documents/report.md";

    request(FilesystemItem {
        index: 7,
        name,
        path,
        kind: FilesystemItemKind::File,
        size: Some(512),
    });
}
