use evo_shell::definitions::contracts::create_dir;
use evo_shell::resolvers::create_dir_resolver;

fn mock_create_dir_success(_target: &str) -> Result<(), create_dir::Error> {
    Ok(())
}

fn mock_create_dir_unavailable(_target: &str) -> Result<(), create_dir::Error> {
    Err(create_dir::Error::Unavailable)
}

#[test]
fn create_dir_resolver_success() {
    assert_eq!(
        create_dir_resolver::resolve(mock_create_dir_success, "documents"),
        Ok(())
    );
}

#[test]
fn create_dir_resolver_translates_error() {
    assert_eq!(
        create_dir_resolver::resolve(mock_create_dir_unavailable, "documents"),
        Err(create_dir_resolver::Error::Unavailable)
    );
}
