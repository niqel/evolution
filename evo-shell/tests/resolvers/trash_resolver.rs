use evo_shell::definitions::contracts::trash;
use evo_shell::resolvers::trash_resolver;

fn mock_trash_success(_target: &str) -> Result<(), trash::Error> {
    Ok(())
}

fn mock_trash_unavailable(_target: &str) -> Result<(), trash::Error> {
    Err(trash::Error::Unavailable)
}

#[test]
fn trash_resolver_success() {
    assert_eq!(
        trash_resolver::resolve(mock_trash_success, "file.txt"),
        Ok(())
    );
}

#[test]
fn trash_resolver_translates_error() {
    assert_eq!(
        trash_resolver::resolve(mock_trash_unavailable, "file.txt"),
        Err(trash_resolver::Error::Unavailable)
    );
}
