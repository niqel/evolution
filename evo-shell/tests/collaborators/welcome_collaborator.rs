use evo_shell::collaborators::welcome_collaborator;
use evo_shell::definitions::structs::borrowed::welcome_information::WelcomeInformation;

fn assert_welcome(welcome: WelcomeInformation<'_>) {
    assert_eq!(welcome.company, "CatarinaSoft");
    assert_eq!(welcome.shell.name, "Evolution Shell");
    assert_eq!(welcome.shell.version, env!("CARGO_PKG_VERSION"));
    assert_eq!(welcome.shell.description, "A lightweight functional shell.");
    assert_eq!(welcome.message, "Evo shell is a life :)");
}

#[test]
fn welcome_collaborator_materializes_and_requests_information() {
    welcome_collaborator::collaborate(assert_welcome);
}
