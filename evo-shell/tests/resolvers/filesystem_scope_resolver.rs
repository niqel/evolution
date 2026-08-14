use evo_shell::definitions::contracts::provide_filesystem_scope;
use evo_shell::definitions::requesters::scope_requester;
use evo_shell::definitions::structs::borrowed::scope::Scope;
use evo_shell::definitions::use_cases::filesystem_scope;
use evo_shell::resolvers::filesystem_scope_resolver;

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

fn fake_provide_error(
    source: &str,
    _request: scope_requester::Request,
) -> Result<(), provide_filesystem_scope::Error> {
    assert_eq!(source, "../documents");

    Err(provide_filesystem_scope::Error::Unavailable)
}

#[test]
fn filesystem_scope_resolver_success() {
    let result = filesystem_scope_resolver::resolve(fake_provide, "../documents", receive_scope);
    assert_eq!(result, Ok(()));
}

#[test]
fn filesystem_scope_resolver_translates_error() {
    let result =
        filesystem_scope_resolver::resolve(fake_provide_error, "../documents", receive_scope);
    assert_eq!(result, Err(filesystem_scope::Error::ScopeUnavailable));
}
