use evo_shell::tools::shell_information;

#[test]
fn shell_information_tool_returns_correct_information() {
    let information = shell_information::get();
    assert_eq!(information.name, "Evolution Shell");
    assert_eq!(information.version, env!("CARGO_PKG_VERSION"));
    assert_eq!(information.description, "A lightweight functional shell.");
}
