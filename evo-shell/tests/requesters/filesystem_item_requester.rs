use evo_shell::definitions::requesters::filesystem_item_requester;
use evo_shell::definitions::structs::borrowed::filesystem_item::FilesystemItem;
use evo_shell::definitions::structs::owned::filesystem_item_kind::FilesystemItemKind;
use evo_shell::definitions::structs::owned::flow::Flow;

fn continue_after_item(item: FilesystemItem<'_>) -> Flow {
    assert_eq!(item.index, 7);
    assert_eq!(item.name, "report.md");
    assert_eq!(item.path, "/home/user/documents/report.md");
    assert_eq!(item.kind, FilesystemItemKind::File);
    assert_eq!(item.size, Some(512));

    Flow::Continue
}

fn stop_after_item(item: FilesystemItem<'_>) -> Flow {
    assert_eq!(item.index, 7);
    assert_eq!(item.name, "report.md");
    assert_eq!(item.path, "/home/user/documents/report.md");
    assert_eq!(item.kind, FilesystemItemKind::File);
    assert_eq!(item.size, Some(512));

    Flow::Stop
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

    assert_eq!(flow, Flow::Continue);
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

    assert_eq!(flow, Flow::Stop);
}
