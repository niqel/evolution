use evo_shell::agents::shell_informant;
use evo_shell::definitions::structs::borrowed::shell_information::ShellInformation;
use evo_shell::definitions::use_cases::inform_shell;

fn assert_information(information: ShellInformation<'_>) {
    assert_eq!(information.name, "Evolution Shell");
    assert_eq!(information.version, env!("CARGO_PKG_VERSION"));
    assert_eq!(information.description, "A lightweight functional shell.");
}

#[test]
fn shell_informant_inform_delivers_correct_information() {
    let inform: inform_shell::Inform = shell_informant::inform;
    inform(assert_information);

    let inform_const: inform_shell::Inform = shell_informant::INFORM;
    inform_const(assert_information);

    shell_informant::inform(assert_information);
}
