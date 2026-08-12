use evo_shell::agents::shell_informant;

#[test]
fn shell_informant_inform_returns_correct_information() {
    let information = shell_informant::inform();
    assert_eq!(information.name, "Evolution Shell");
    assert_eq!(information.version, env!("CARGO_PKG_VERSION"));
    assert_eq!(information.description, "A lightweight functional shell.");
}
