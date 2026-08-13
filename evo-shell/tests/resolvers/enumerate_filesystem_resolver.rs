use evo_shell::definitions::contracts::enumerate_filesystem as enumerate_filesystem_contract;
use evo_shell::definitions::requesters::filesystem_item_requester;
use evo_shell::definitions::structs::borrowed::filesystem_item::FilesystemItem;
use evo_shell::definitions::structs::borrowed::scope::Scope;
use evo_shell::definitions::structs::filesystem_item_kind::FilesystemItemKind;
use evo_shell::definitions::use_cases::enumerate_filesystem;
use evo_shell::resolvers::enumerate_filesystem_resolver;

fn receive_item(item: FilesystemItem<'_>) -> filesystem_item_requester::Flow {
    assert_eq!(item.index, 3);
    assert_eq!(item.name, "report.md");
    assert_eq!(item.path, "/documents/report.md");
    assert_eq!(item.kind, FilesystemItemKind::File);
    assert_eq!(item.size, Some(256));

    filesystem_item_requester::Flow::Continue
}

fn fake_enumerate_success(
    source: &str,
    request: filesystem_item_requester::Request,
) -> Result<(), enumerate_filesystem_contract::Error> {
    assert_eq!(source, "/documents");

    let flow = request(FilesystemItem {
        index: 3,
        name: "report.md",
        path: "/documents/report.md",
        kind: FilesystemItemKind::File,
        size: Some(256),
    });

    assert_eq!(flow, filesystem_item_requester::Flow::Continue);

    Ok(())
}

fn fake_enumerate_unavailable(
    source: &str,
    _request: filesystem_item_requester::Request,
) -> Result<(), enumerate_filesystem_contract::Error> {
    assert_eq!(source, "/documents");

    Err(enumerate_filesystem_contract::Error::Unavailable)
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
fn enumerate_filesystem_resolver_success() {
    let scope = Scope {
        scope_type: "fs",
        server: "",
        user: "gustavo",
        source: "/documents",
        item: None,
    };

    enumerate_filesystem_resolver::resolve(
        fake_enumerate_success,
        scope,
        receive_item,
        receive_success,
    );
}

#[test]
fn enumerate_filesystem_resolver_translates_error() {
    let scope = Scope {
        scope_type: "fs",
        server: "",
        user: "gustavo",
        source: "/documents",
        item: None,
    };

    enumerate_filesystem_resolver::resolve(
        fake_enumerate_unavailable,
        scope,
        receive_item,
        receive_error,
    );
}
