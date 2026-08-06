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
