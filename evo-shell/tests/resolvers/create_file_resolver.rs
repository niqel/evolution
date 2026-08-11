use evo_shell::definitions::contracts::create_file;
use evo_shell::resolvers::create_file_resolver;

fn mock_create_file_success(_target: &str) -> Result<(), create_file::Error> {
    Ok(())
}

fn mock_create_file_unavailable(_target: &str) -> Result<(), create_file::Error> {
    Err(create_file::Error::Unavailable)
}

#[test]
fn create_file_resolver_success() {
    assert_eq!(
        create_file_resolver::resolve(mock_create_file_success, "notes.txt"),
        Ok(())
    );
}

#[test]
fn create_file_resolver_translates_error() {
    assert_eq!(
        create_file_resolver::resolve(mock_create_file_unavailable, "notes.txt"),
        Err(create_file_resolver::Error::Unavailable)
    );
}
