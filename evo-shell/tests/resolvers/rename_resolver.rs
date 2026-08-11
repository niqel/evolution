use evo_shell::definitions::contracts::rename;
use evo_shell::resolvers::rename_resolver;

fn mock_rename_success(_target: &str, _new_name: &str) -> Result<(), rename::Error> {
    Ok(())
}

fn mock_rename_unavailable(_target: &str, _new_name: &str) -> Result<(), rename::Error> {
    Err(rename::Error::Unavailable)
}

#[test]
fn rename_resolver_success() {
    assert_eq!(
        rename_resolver::resolve(mock_rename_success, "videos/gatito.mp4", "michi.mp4"),
        Ok(())
    );
}

#[test]
fn rename_resolver_translates_error() {
    assert_eq!(
        rename_resolver::resolve(mock_rename_unavailable, "videos/gatito.mp4", "michi.mp4"),
        Err(rename_resolver::Error::Unavailable)
    );
}
