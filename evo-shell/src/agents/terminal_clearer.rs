use crate::definitions::domain::value_objects::terminal_clear_mode::TerminalClearMode;
use crate::definitions::providers::terminal_clearer::Provide;
use crate::definitions::resolvers::terminal_clearer::Resolve;
use crate::definitions::use_cases::terminal_clearer::TerminalClearError;
use crate::providers::terminal_clearer as provider;
use crate::resolvers::terminal_clearer as resolver;

pub fn clear(mode: TerminalClearMode) -> Result<(), TerminalClearError> {
    let resolve: Resolve = resolver::resolve;
    let provide: Provide = provider::provide;

    clear_with(mode, resolve, provide)
}

pub(crate) fn clear_with(
    mode: TerminalClearMode,
    resolve: Resolve,
    provide: Provide,
) -> Result<(), TerminalClearError> {
    resolve(mode, provide)
}
