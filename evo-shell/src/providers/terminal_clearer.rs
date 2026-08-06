use std::io::{self, Write};

use crate::definitions::use_cases::terminal_clearer::TerminalClearError;

const CLEAR_SEQUENCE: &str = "\x1b[2J\x1b[3J\x1b[H";

pub fn provide() -> Result<(), TerminalClearError> {
    let mut stdout = io::stdout();
    provide_to(&mut stdout)?;
    stdout.flush().map_err(TerminalClearError::from)
}

pub(crate) fn provide_to(writer: &mut impl Write) -> Result<(), TerminalClearError> {
    writer
        .write_all(CLEAR_SEQUENCE.as_bytes())
        .map_err(TerminalClearError::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FailingWriter;

    impl io::Write for FailingWriter {
        fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
            Err(io::Error::other("write failed"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn clear_uses_full_terminal_clear_sequence() {
        let mut output = Vec::new();

        provide_to(&mut output).unwrap();

        assert_eq!(output, b"\x1b[2J\x1b[3J\x1b[H");
    }

    #[test]
    fn clear_no_longer_uses_visible_only_sequence() {
        let mut output = Vec::new();

        provide_to(&mut output).unwrap();

        assert_ne!(output, b"\x1b[2J\x1b[H");
    }

    #[test]
    fn terminal_clearer_provider_propagates_io_error() {
        let mut writer = FailingWriter;

        let result = provide_to(&mut writer);

        assert!(matches!(result, Err(TerminalClearError::Io(_))));
    }
}
