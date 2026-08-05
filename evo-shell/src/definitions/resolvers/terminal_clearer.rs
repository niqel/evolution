use crate::definitions::domain::value_objects::terminal_clear_mode::TerminalClearMode;
use crate::definitions::providers::terminal_clearer::Provide;
use crate::definitions::use_cases::terminal_clearer::TerminalClearError;

pub type Resolve = fn(TerminalClearMode, Provide) -> Result<(), TerminalClearError>;
