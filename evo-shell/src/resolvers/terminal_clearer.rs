use crate::definitions::providers::terminal_clearer::Provide;
use crate::definitions::use_cases::terminal_clearer::TerminalClearError;

pub fn resolve(provide: Provide) -> Result<(), TerminalClearError> {
    provide()
}
