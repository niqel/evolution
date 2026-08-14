use evo_shell::definitions::contracts::enter_scope as enter_scope_contract;
use evo_shell::definitions::use_cases::enter_scope;
use evo_shell::resolvers::enter_scope_resolver;

fn fake_enter(target: &str) -> Result<(), enter_scope_contract::Error> {
    assert_eq!(target, "..");

    Ok(())
}

fn fake_enter_unavailable(target: &str) -> Result<(), enter_scope_contract::Error> {
    assert_eq!(target, "workers");

    Err(enter_scope_contract::Error::Unavailable)
}

#[test]
fn enter_scope_resolver_success() {
    let result = enter_scope_resolver::resolve(fake_enter, "..");
    assert_eq!(result, Ok(()));
}

#[test]
fn enter_scope_resolver_translates_error() {
    let result = enter_scope_resolver::resolve(fake_enter_unavailable, "workers");
    assert_eq!(result, Err(enter_scope::Error::NavigationUnavailable));
}
