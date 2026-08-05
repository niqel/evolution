use crate::definitions::domain::value_objects::terminal_clear_mode::TerminalClearMode;
use crate::definitions::use_cases::terminal_clearer::TerminalClearError;

pub type Provide = fn(TerminalClearMode) -> Result<(), TerminalClearError>;
