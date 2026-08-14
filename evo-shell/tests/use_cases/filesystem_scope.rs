use evo_shell::definitions::contracts::provide_filesystem_scope;
use evo_shell::definitions::requesters::scope_requester;
use evo_shell::definitions::structs::borrowed::scope::Scope;
use evo_shell::definitions::use_cases::filesystem_scope;

fn receive_scope(scope: Scope<'_>) {
    assert_eq!(scope.scope_type, "fs");
    assert_eq!(scope.server, "");
    assert_eq!(scope.user, "gustavo");
    assert_eq!(scope.source, "/home/gustavo/documents");
    assert_eq!(scope.item, Some("documents"));
}

fn fake_contract(
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

fn fake_contract_error(
    source: &str,
    _request: scope_requester::Request,
) -> Result<(), provide_filesystem_scope::Error> {
    assert_eq!(source, "../documents");

    Err(provide_filesystem_scope::Error::Unavailable)
}

fn fake_use_case(
    source: &str,
    request: scope_requester::Request,
    provide: provide_filesystem_scope::Provide,
) -> Result<(), filesystem_scope::Error> {
    provide(source, request).map_err(|_| filesystem_scope::Error::ScopeUnavailable)
}

#[test]
fn filesystem_scope_use_case_signature_and_success() {
    let use_case: filesystem_scope::Provide = fake_use_case;

    let result = use_case("../documents", receive_scope, fake_contract);
    assert_eq!(result, Ok(()));
}

#[test]
fn filesystem_scope_use_case_error() {
    let use_case: filesystem_scope::Provide = fake_use_case;

    let result = use_case("../documents", receive_scope, fake_contract_error);
    assert_eq!(result, Err(filesystem_scope::Error::ScopeUnavailable));
}
