use std::io::{self, Write};

use crate::definitions::domain::value_objects::terminal_clear_mode::TerminalClearMode;
use crate::definitions::use_cases::terminal_clearer::TerminalClearError;

const CLEAR_VISIBLE: &str = "\x1b[2J\x1b[H";
const CLEAR_ALL: &str = "\x1b[2J\x1b[3J\x1b[H";

pub fn provide(mode: TerminalClearMode) -> Result<(), TerminalClearError> {
    let mut stdout = io::stdout();
    provide_to(&mut stdout, mode)?;
    stdout.flush().map_err(TerminalClearError::from)
}

pub(crate) fn provide_to(
    writer: &mut impl Write,
    mode: TerminalClearMode,
) -> Result<(), TerminalClearError> {
    let sequence = match mode {
        TerminalClearMode::Visible => CLEAR_VISIBLE,
        TerminalClearMode::All => CLEAR_ALL,
    };

    writer
        .write_all(sequence.as_bytes())
        .map_err(TerminalClearError::from)
}
