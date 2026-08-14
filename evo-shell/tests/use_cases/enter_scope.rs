use evo_shell::definitions::contracts::enter_scope as enter_scope_contract;
use evo_shell::definitions::use_cases::enter_scope;

fn fake_contract(target: &str) -> Result<(), enter_scope_contract::Error> {
    assert_eq!(target, "..");

    Ok(())
}

fn fake_contract_unavailable(target: &str) -> Result<(), enter_scope_contract::Error> {
    assert_eq!(target, "workers");

    Err(enter_scope_contract::Error::Unavailable)
}

fn fake_use_case(
    target: &str,
    enter: enter_scope_contract::Enter,
) -> Result<(), enter_scope::Error> {
    enter(target).map_err(|_| enter_scope::Error::NavigationUnavailable)
}

#[test]
fn enter_scope_use_case_signature_and_success() {
    let use_case: enter_scope::Enter = fake_use_case;

    let result = use_case("..", fake_contract);

    assert_eq!(result, Ok(()));
}

#[test]
fn enter_scope_use_case_error() {
    let use_case: enter_scope::Enter = fake_use_case;

    let result = use_case("workers", fake_contract_unavailable);

    assert_eq!(result, Err(enter_scope::Error::NavigationUnavailable));
}
