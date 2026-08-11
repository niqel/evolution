use evo_shell::definitions::contracts::delete;
use evo_shell::resolvers::delete_resolver;

fn mock_delete_success(_target: &str) -> Result<(), delete::Error> {
    Ok(())
}

fn mock_delete_unavailable(_target: &str) -> Result<(), delete::Error> {
    Err(delete::Error::Unavailable)
}

#[test]
fn delete_resolver_success() {
    assert_eq!(
        delete_resolver::resolve(mock_delete_success, "file.txt"),
        Ok(())
    );
}

#[test]
fn delete_resolver_translates_error() {
    assert_eq!(
        delete_resolver::resolve(mock_delete_unavailable, "file.txt"),
        Err(delete_resolver::Error::Unavailable)
    );
}
