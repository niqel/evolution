use crate::definitions::contracts::write_terminal;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub fn resolve(write: write_terminal::Write, content: &str) -> Result<(), Error> {
    write(content).map_err(|write_terminal::Error::Unavailable| Error::Unavailable)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_write_success(_content: &str) -> Result<(), write_terminal::Error> {
        Ok(())
    }

    fn mock_write_failure(_content: &str) -> Result<(), write_terminal::Error> {
        Err(write_terminal::Error::Unavailable)
    }

    #[test]
    fn terminal_writer_success() {
        assert_eq!(resolve(mock_write_success, "hello"), Ok(()));
    }

    #[test]
    fn terminal_writer_translates_error() {
        assert_eq!(
            resolve(mock_write_failure, "hello"),
            Err(Error::Unavailable)
        );
    }
}
