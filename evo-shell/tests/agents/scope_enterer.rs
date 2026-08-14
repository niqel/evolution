use evo_shell::agents::scope_enterer;
use evo_shell::definitions::contracts::enter_scope as enter_scope_contract;
use evo_shell::definitions::use_cases::enter_scope;

fn fake_enter(target: &str) -> Result<(), enter_scope_contract::Error> {
    assert_eq!(target, "..");

    Ok(())
}

fn fake_enter_unavailable(target: &str) -> Result<(), enter_scope_contract::Error> {
    assert_eq!(target, "workers");

    Err(enter_scope_contract::Error::Unavailable)
}

#[test]
fn scope_enterer_success() {
    let agent: enter_scope::Enter = scope_enterer::ENTER;

    let result = agent("..", fake_enter);

    assert_eq!(result, Ok(()));
}

#[test]
fn scope_enterer_translates_error() {
    let agent: enter_scope::Enter = scope_enterer::ENTER;

    let result = agent("workers", fake_enter_unavailable);

    assert_eq!(result, Err(enter_scope::Error::NavigationUnavailable));
}
