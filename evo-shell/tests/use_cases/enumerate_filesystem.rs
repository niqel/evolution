use evo_shell::definitions::contracts::enumerate_filesystem as enumerate_filesystem_contract;
use evo_shell::definitions::requesters::enumerate_filesystem_requester;
use evo_shell::definitions::requesters::filesystem_item_requester;
use evo_shell::definitions::structs::borrowed::filesystem_item::FilesystemItem;
use evo_shell::definitions::structs::borrowed::scope::Scope;
use evo_shell::definitions::structs::owned::flow::Flow;
use evo_shell::definitions::use_cases::enumerate_filesystem;

fn fake_contract(
    _source: &str,
    _request: filesystem_item_requester::Request,
) -> Result<(), enumerate_filesystem_contract::Error> {
    Ok(())
}

fn receive_item(_item: FilesystemItem<'_>) -> Flow {
    Flow::Continue
}

fn receive_result(_result: Result<(), enumerate_filesystem::Error>) {}

fn fake_use_case(
    _scope: Scope<'_>,
    _item_request: filesystem_item_requester::Request,
    _result_request: enumerate_filesystem_requester::Request,
    _enumerate: enumerate_filesystem_contract::Enumerate,
) {
}

#[test]
fn enumerate_filesystem_use_case_signature() {
    let enumerate: enumerate_filesystem::Enumerate = fake_use_case;

    let scope = Scope {
        scope_type: "fs",
        server: "",
        user: "gustavo",
        source: "/home/gustavo/documents",
        item: Some("documents"),
    };

    enumerate(scope, receive_item, receive_result, fake_contract);
}
