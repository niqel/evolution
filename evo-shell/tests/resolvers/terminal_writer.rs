use evo_shell::definitions::contracts::write_terminal;
use evo_shell::resolvers::terminal_writer;

fn mock_write_success(_content: &str) -> Result<(), write_terminal::Error> {
    Ok(())
}

fn mock_write_failure(_content: &str) -> Result<(), write_terminal::Error> {
    Err(write_terminal::Error::Unavailable)
}

#[test]
fn terminal_writer_success() {
    assert_eq!(
        terminal_writer::resolve(mock_write_success, "hello"),
        Ok(())
    );
}

#[test]
fn terminal_writer_translates_error() {
    assert_eq!(
        terminal_writer::resolve(mock_write_failure, "hello"),
        Err(terminal_writer::Error::Unavailable)
    );
}
