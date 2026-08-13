use evo_shell::agents::filesystem_enumerator;
use evo_shell::definitions::contracts::enumerate_filesystem as enumerate_filesystem_contract;
use evo_shell::definitions::requesters::filesystem_item_requester;
use evo_shell::definitions::structs::borrowed::filesystem_item::FilesystemItem;
use evo_shell::definitions::structs::borrowed::scope::Scope;
use evo_shell::definitions::structs::filesystem_item_kind::FilesystemItemKind;
use evo_shell::definitions::use_cases::enumerate_filesystem;

fn fake_contract_success(
    source: &str,
    request: filesystem_item_requester::Request,
) -> Result<(), enumerate_filesystem_contract::Error> {
    assert_eq!(source, "/documents");

    let flow = request(FilesystemItem {
        index: 1,
        name: "report.md",
        path: "/documents/report.md",
        kind: FilesystemItemKind::File,
        size: Some(128),
    });

    assert_eq!(flow, filesystem_item_requester::Flow::Continue);

    Ok(())
}

fn fake_contract_error(
    source: &str,
    _request: filesystem_item_requester::Request,
) -> Result<(), enumerate_filesystem_contract::Error> {
    assert_eq!(source, "/documents");

    Err(enumerate_filesystem_contract::Error::Unavailable)
}

fn receive_item(item: FilesystemItem<'_>) -> filesystem_item_requester::Flow {
    assert_eq!(item.index, 1);
    assert_eq!(item.name, "report.md");
    assert_eq!(item.path, "/documents/report.md");
    assert_eq!(item.kind, FilesystemItemKind::File);
    assert_eq!(item.size, Some(128));

    filesystem_item_requester::Flow::Continue
}

fn receive_success(result: Result<(), enumerate_filesystem::Error>) {
    assert_eq!(result, Ok(()));
}

fn receive_error(result: Result<(), enumerate_filesystem::Error>) {
    assert_eq!(
        result,
        Err(enumerate_filesystem::Error::EnumerationUnavailable)
    );
}

#[test]
fn filesystem_enumerator_success() {
    let enumerate_op: enumerate_filesystem::Enumerate = filesystem_enumerator::enumerate;
    let enumerate_const: enumerate_filesystem::Enumerate = filesystem_enumerator::ENUMERATE;

    let scope = Scope {
        scope_type: "fs",
        server: "",
        user: "gustavo",
        source: "/documents",
        item: None,
    };

    enumerate_op(scope, receive_item, receive_success, fake_contract_success);
    enumerate_const(scope, receive_item, receive_success, fake_contract_success);
}

#[test]
fn filesystem_enumerator_error() {
    let scope = Scope {
        scope_type: "fs",
        server: "",
        user: "gustavo",
        source: "/documents",
        item: None,
    };

    filesystem_enumerator::ENUMERATE(scope, receive_item, receive_error, fake_contract_error);
}
