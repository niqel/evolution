use evo_shell::definitions::requesters::filesystem_item_requester;
use evo_shell::definitions::structs::borrowed::filesystem_item::FilesystemItem;
use evo_shell::definitions::structs::filesystem_item_kind::FilesystemItemKind;

fn continue_after_item(item: FilesystemItem<'_>) -> filesystem_item_requester::Flow {
    assert_eq!(item.index, 7);
    assert_eq!(item.name, "report.md");
    assert_eq!(item.path, "/home/user/documents/report.md");
    assert_eq!(item.kind, FilesystemItemKind::File);
    assert_eq!(item.size, Some(512));

    filesystem_item_requester::Flow::Continue
}

fn stop_after_item(item: FilesystemItem<'_>) -> filesystem_item_requester::Flow {
    assert_eq!(item.index, 7);
    assert_eq!(item.name, "report.md");
    assert_eq!(item.path, "/home/user/documents/report.md");
    assert_eq!(item.kind, FilesystemItemKind::File);
    assert_eq!(item.size, Some(512));

    filesystem_item_requester::Flow::Stop
}

#[test]
fn filesystem_item_requester_returns_flow_continue() {
    let request: filesystem_item_requester::Request = continue_after_item;

    let name = "report.md";
    let path = "/home/user/documents/report.md";

    let flow = request(FilesystemItem {
        index: 7,
        name,
        path,
        kind: FilesystemItemKind::File,
        size: Some(512),
    });

    assert_eq!(flow, filesystem_item_requester::Flow::Continue);
}

#[test]
fn filesystem_item_requester_returns_flow_stop() {
    let request: filesystem_item_requester::Request = stop_after_item;

    let name = "report.md";
    let path = "/home/user/documents/report.md";

    let flow = request(FilesystemItem {
        index: 7,
        name,
        path,
        kind: FilesystemItemKind::File,
        size: Some(512),
    });

    assert_eq!(flow, filesystem_item_requester::Flow::Stop);
}
