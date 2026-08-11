use evo_shell::definitions::contracts::copy;
use evo_shell::resolvers::copy_resolver;

fn mock_copy_success(_origin: &str, _destination: &str) -> Result<(), copy::Error> {
    Ok(())
}

fn mock_copy_unavailable(_origin: &str, _destination: &str) -> Result<(), copy::Error> {
    Err(copy::Error::Unavailable)
}

#[test]
fn copy_resolver_success() {
    assert_eq!(
        copy_resolver::resolve(mock_copy_success, "origin.txt", "../documents"),
        Ok(())
    );
}

#[test]
fn copy_resolver_translates_error() {
    assert_eq!(
        copy_resolver::resolve(mock_copy_unavailable, "origin.txt", "../documents"),
        Err(copy_resolver::Error::Unavailable)
    );
}
