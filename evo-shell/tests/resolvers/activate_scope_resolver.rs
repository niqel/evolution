use evo_shell::definitions::contracts::activate_scope as activate_scope_contract;
use evo_shell::definitions::structs::borrowed::scope::Scope;
use evo_shell::definitions::use_cases::activate_scope;
use evo_shell::resolvers::activate_scope_resolver;

fn fake_activate(scope: Scope<'_>) -> Result<(), activate_scope_contract::Error> {
    assert_eq!(scope.scope_type, "fs");
    assert_eq!(scope.server, "");
    assert_eq!(scope.user, "gustavo");
    assert_eq!(scope.source, "/home/gustavo/documents");
    assert_eq!(scope.item, Some("documents"));

    Ok(())
}

fn fake_activate_unavailable(_scope: Scope<'_>) -> Result<(), activate_scope_contract::Error> {
    Err(activate_scope_contract::Error::Unavailable)
}

#[test]
fn activate_scope_resolver_success() {
    let result = activate_scope_resolver::resolve(
        fake_activate,
        Scope {
            scope_type: "fs",
            server: "",
            user: "gustavo",
            source: "/home/gustavo/documents",
            item: Some("documents"),
        },
    );
    assert_eq!(result, Ok(()));
}

#[test]
fn activate_scope_resolver_translates_error() {
    let result = activate_scope_resolver::resolve(
        fake_activate_unavailable,
        Scope {
            scope_type: "fs",
            server: "",
            user: "gustavo",
            source: "/home/gustavo/documents",
            item: Some("documents"),
        },
    );
    assert_eq!(result, Err(activate_scope::Error::ActivationUnavailable));
}
