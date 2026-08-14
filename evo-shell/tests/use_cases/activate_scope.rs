use evo_shell::definitions::contracts::activate_scope as activate_scope_contract;
use evo_shell::definitions::structs::borrowed::scope::Scope;
use evo_shell::definitions::use_cases::activate_scope;

fn fake_contract(scope: Scope<'_>) -> Result<(), activate_scope_contract::Error> {
    assert_eq!(scope.scope_type, "fs");
    assert_eq!(scope.server, "");
    assert_eq!(scope.user, "gustavo");
    assert_eq!(scope.source, "/home/gustavo/documents");
    assert_eq!(scope.item, Some("documents"));

    Ok(())
}

fn fake_contract_unavailable(_scope: Scope<'_>) -> Result<(), activate_scope_contract::Error> {
    Err(activate_scope_contract::Error::Unavailable)
}

fn fake_use_case(
    scope: Scope<'_>,
    activate: activate_scope_contract::Activate,
) -> Result<(), activate_scope::Error> {
    activate(scope).map_err(|_| activate_scope::Error::ActivationUnavailable)
}

#[test]
fn activate_scope_use_case_signature_and_success() {
    let use_case: activate_scope::Activate = fake_use_case;

    let result = use_case(
        Scope {
            scope_type: "fs",
            server: "",
            user: "gustavo",
            source: "/home/gustavo/documents",
            item: Some("documents"),
        },
        fake_contract,
    );

    assert_eq!(result, Ok(()));
}

#[test]
fn activate_scope_use_case_error() {
    let use_case: activate_scope::Activate = fake_use_case;

    let result = use_case(
        Scope {
            scope_type: "fs",
            server: "",
            user: "gustavo",
            source: "/home/gustavo/documents",
            item: Some("documents"),
        },
        fake_contract_unavailable,
    );

    assert_eq!(result, Err(activate_scope::Error::ActivationUnavailable));
}
