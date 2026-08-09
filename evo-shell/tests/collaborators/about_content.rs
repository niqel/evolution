use evo_shell::collaborators::about_content;

#[test]
fn version_matches_cargo_pkg_version() {
    assert_eq!(about_content::VERSION, env!("CARGO_PKG_VERSION"));
}

#[test]
fn static_content_invariants() {
    assert_eq!(about_content::NAME, "Evolution Shell");
    assert_eq!(
        about_content::DESCRIPTION,
        "A lightweight functional shell."
    );
}
