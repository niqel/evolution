use evo_shell::agents::about_responder;
use evo_shell::definitions::structs::borrowed::shell_information::ShellInformation;
use evo_shell::definitions::use_cases::respond_about;

fn assert_about(about: ShellInformation<'_>) {
    assert_eq!(about.name, "Evolution Shell");
    assert_eq!(about.version, env!("CARGO_PKG_VERSION"));
    assert_eq!(about.description, "A lightweight functional shell.");
}

#[test]
fn about_responder_implements_respond_about() {
    let respond: respond_about::Respond = about_responder::respond;
    respond(assert_about);
}
