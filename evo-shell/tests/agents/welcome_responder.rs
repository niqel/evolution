use evo_shell::agents::welcome_responder;
use evo_shell::definitions::structs::borrowed::welcome_information::WelcomeInformation;
use evo_shell::definitions::use_cases::respond_welcome;

fn assert_welcome(welcome: WelcomeInformation<'_>) {
    assert_eq!(welcome.company, "CatarinaSoft");
    assert_eq!(welcome.shell.name, "Evolution Shell");
    assert_eq!(welcome.shell.version, env!("CARGO_PKG_VERSION"));
    assert_eq!(welcome.shell.description, "A lightweight functional shell.");
    assert_eq!(welcome.message, "Evo shell is a life :)");
}

#[test]
fn welcome_responder_implements_respond_welcome() {
    let respond: respond_welcome::Respond = welcome_responder::respond;
    respond(assert_welcome);

    let respond_const: respond_welcome::Respond = welcome_responder::RESPOND;
    respond_const(assert_welcome);
}
