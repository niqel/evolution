use evo_shell::collaborators::about_collaborator;
use evo_shell::definitions::structs::borrowed::shell_information::ShellInformation;

fn assert_about(about: ShellInformation<'_>) {
    assert_eq!(about.name, "Evolution Shell");
    assert_eq!(about.version, env!("CARGO_PKG_VERSION"));
    assert_eq!(about.description, "A lightweight functional shell.");
}

#[test]
fn about_collaborator_materializes_and_requests_information() {
    about_collaborator::collaborate(assert_about);
}
