use evo_shell::agents::scope_responder;
use evo_shell::definitions::contracts::provide_scope;
use evo_shell::definitions::requesters::scope_requester;
use evo_shell::definitions::structs::borrowed::scope::Scope;
use evo_shell::definitions::use_cases::respond_scope;

fn mock_provide_success(request: scope_requester::Request) -> Result<(), provide_scope::Error> {
    let scope = Scope {
        scope_type: "fs",
        server: "test-server",
        user: "test-user",
        source: "/",
        item: Some("/downloads"),
    };

    request(scope);

    Ok(())
}

fn mock_provide_error(_request: scope_requester::Request) -> Result<(), provide_scope::Error> {
    Err(provide_scope::Error::Unavailable)
}

fn mock_request(_scope: Scope<'_>) {}

#[test]
fn scope_responder_responds_successfully() {
    assert_eq!(
        scope_responder::respond(mock_provide_success, mock_request),
        Ok(())
    );
}

#[test]
fn scope_responder_returns_semantic_scope_error() {
    assert_eq!(
        scope_responder::respond(mock_provide_error, mock_request),
        Err(respond_scope::Error::ScopeUnavailable)
    );
}
