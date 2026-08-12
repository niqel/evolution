use evo_shell::collaborators::shell_welcomer;
use evo_shell::definitions::structs::borrowed::shell_information::ShellInformation;

#[test]
fn shell_welcomer_collaborate_returns_correct_information() {
    let dummy_shell = ShellInformation {
        name: "Test Shell",
        version: "1.0.0",
        description: "Test description",
    };

    let welcome = shell_welcomer::collaborate(dummy_shell);

    assert_eq!(welcome.company, "CatarinaSoft");
    assert_eq!(welcome.shell, dummy_shell);
    assert_eq!(welcome.message, "Evo shell is a life :)");
}
