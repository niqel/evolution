use crate::definitions::providers::terminal_clearer::Provide;
use crate::definitions::use_cases::terminal_clearer::TerminalClearError;

pub type Resolve = fn(Provide) -> Result<(), TerminalClearError>;
