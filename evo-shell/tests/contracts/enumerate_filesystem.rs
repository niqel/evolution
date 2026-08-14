use evo_shell::definitions::contracts::enumerate_filesystem;
use evo_shell::definitions::requesters::filesystem_item_requester;
use evo_shell::definitions::structs::borrowed::filesystem_item::FilesystemItem;
use evo_shell::definitions::structs::owned::filesystem_item_kind::FilesystemItemKind;
use evo_shell::definitions::structs::owned::flow::Flow;

fn receive_item_stop(item: FilesystemItem<'_>) -> Flow {
    assert_eq!(item.index, 0);
    assert_eq!(item.name, "report.md");
    assert_eq!(item.path, "/documents/report.md");
    assert_eq!(item.kind, FilesystemItemKind::File);
    assert_eq!(item.size, Some(128));

    Flow::Stop
}

fn fake_enumerate(
    source: &str,
    request: filesystem_item_requester::Request,
) -> Result<(), enumerate_filesystem::Error> {
    assert_eq!(source, "/documents");

    let flow = request(FilesystemItem {
        index: 0,
        name: "report.md",
        path: "/documents/report.md",
        kind: FilesystemItemKind::File,
        size: Some(128),
    });

    assert_eq!(flow, Flow::Stop);

    Ok(())
}

fn unavailable_enumerate(
    _source: &str,
    _request: filesystem_item_requester::Request,
) -> Result<(), enumerate_filesystem::Error> {
    Err(enumerate_filesystem::Error::Unavailable)
}

#[test]
fn enumerate_filesystem_contract_signature_and_success() {
    let enumerate: enumerate_filesystem::Enumerate = fake_enumerate;
    assert_eq!(enumerate("/documents", receive_item_stop), Ok(()));
}

#[test]
fn enumerate_filesystem_contract_error() {
    let enumerate: enumerate_filesystem::Enumerate = unavailable_enumerate;
    assert_eq!(
        enumerate("/documents", receive_item_stop),
        Err(enumerate_filesystem::Error::Unavailable)
    );
}
