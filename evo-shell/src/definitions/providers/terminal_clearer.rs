use crate::definitions::use_cases::terminal_clearer::TerminalClearError;

pub type Provide = fn() -> Result<(), TerminalClearError>;
