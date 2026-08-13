use evo_shell::collaborators::shell_informant;
use evo_shell::definitions::structs::borrowed::shell_information::ShellInformation;

fn assert_information(information: ShellInformation<'_>) {
    assert_eq!(information.name, "Evolution Shell");
    assert_eq!(information.version, env!("CARGO_PKG_VERSION"));
    assert_eq!(information.description, "A lightweight functional shell.");
}

#[test]
fn shell_informant_collaborate_delivers_correct_information() {
    shell_informant::collaborate(assert_information);
}
