use evo_shell::collaborators::shell_informant;

#[test]
fn shell_informant_collaborate_returns_correct_information() {
    let information = shell_informant::collaborate();
    assert_eq!(information.name, "Evolution Shell");
    assert_eq!(information.version, env!("CARGO_PKG_VERSION"));
    assert_eq!(information.description, "A lightweight functional shell.");
}
