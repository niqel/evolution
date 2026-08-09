use evo_shell::collaborators::welcome_content;

#[test]
fn static_welcome_content_invariants() {
    assert_eq!(welcome_content::COMPANY, "CatarinaSoft");
    assert_eq!(welcome_content::MESSAGE, "Evo shell is a life :)");
}
