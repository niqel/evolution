use evo_shell::definitions::contracts::provide_filesystem_scope;
use evo_shell::definitions::requesters::scope_requester;
use evo_shell::definitions::structs::borrowed::scope::Scope;

fn receive_scope(scope: Scope<'_>) {
    assert_eq!(scope.scope_type, "fs");
    assert_eq!(scope.server, "");
    assert_eq!(scope.user, "gustavo");
    assert_eq!(scope.source, "/home/gustavo/documents");
    assert_eq!(scope.item, Some("documents"));
}

fn fake_provide(
    source: &str,
    request: scope_requester::Request,
) -> Result<(), provide_filesystem_scope::Error> {
    assert_eq!(source, "../documents");

    request(Scope {
        scope_type: "fs",
        server: "",
        user: "gustavo",
        source: "/home/gustavo/documents",
        item: Some("documents"),
    });

    Ok(())
}

fn fake_provide_unavailable(
    _source: &str,
    _request: scope_requester::Request,
) -> Result<(), provide_filesystem_scope::Error> {
    Err(provide_filesystem_scope::Error::Unavailable)
}

#[test]
fn provide_filesystem_scope_contract_success() {
    let provide: provide_filesystem_scope::Provide = fake_provide;

    let result = provide("../documents", receive_scope);
    assert_eq!(result, Ok(()));
}

#[test]
fn provide_filesystem_scope_contract_error() {
    let provide: provide_filesystem_scope::Provide = fake_provide_unavailable;

    let result = provide("../documents", receive_scope);
    assert_eq!(result, Err(provide_filesystem_scope::Error::Unavailable));
}
