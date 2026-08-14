use evo_shell::definitions::contracts::activate_scope;
use evo_shell::definitions::structs::borrowed::scope::Scope;

fn fake_activate(scope: Scope<'_>) -> Result<(), activate_scope::Error> {
    assert_eq!(scope.scope_type, "fs");
    assert_eq!(scope.server, "");
    assert_eq!(scope.user, "gustavo");
    assert_eq!(scope.source, "/home/gustavo/documents");
    assert_eq!(scope.item, Some("documents"));

    Ok(())
}

fn fake_activate_unavailable(_scope: Scope<'_>) -> Result<(), activate_scope::Error> {
    Err(activate_scope::Error::Unavailable)
}

#[test]
fn activate_scope_contract_success() {
    let activate: activate_scope::Activate = fake_activate;

    let result = activate(Scope {
        scope_type: "fs",
        server: "",
        user: "gustavo",
        source: "/home/gustavo/documents",
        item: Some("documents"),
    });

    assert_eq!(result, Ok(()));
}

#[test]
fn activate_scope_contract_error() {
    let activate: activate_scope::Activate = fake_activate_unavailable;

    let result = activate(Scope {
        scope_type: "fs",
        server: "",
        user: "gustavo",
        source: "/home/gustavo/documents",
        item: Some("documents"),
    });

    assert_eq!(result, Err(activate_scope::Error::Unavailable));
}
