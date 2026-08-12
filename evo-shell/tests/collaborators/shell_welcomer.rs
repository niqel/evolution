use evo_shell::collaborators::shell_welcomer;

#[test]
fn shell_welcomer_collaborate_returns_correct_information() {
    let welcome = shell_welcomer::collaborate();

    assert_eq!(welcome.company, "CatarinaSoft");
    assert_eq!(welcome.shell.name, "Evolution Shell");
    assert_eq!(welcome.shell.version, env!("CARGO_PKG_VERSION"));
    assert_eq!(welcome.shell.description, "A lightweight functional shell.");
    assert_eq!(welcome.message, "Evo shell is a life :)");
}
