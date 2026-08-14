use evo_shell::definitions::contracts::enter_scope;

fn fake_enter(target: &str) -> Result<(), enter_scope::Error> {
    assert_eq!(target, "..");

    Ok(())
}

fn fake_enter_unavailable(target: &str) -> Result<(), enter_scope::Error> {
    assert_eq!(target, "workers");

    Err(enter_scope::Error::Unavailable)
}

#[test]
fn enter_scope_contract_success() {
    let enter: enter_scope::Enter = fake_enter;

    let result = enter("..");

    assert_eq!(result, Ok(()));
}

#[test]
fn enter_scope_contract_error() {
    let enter: enter_scope::Enter = fake_enter_unavailable;

    let result = enter("workers");

    assert_eq!(result, Err(enter_scope::Error::Unavailable));
}
