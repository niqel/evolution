use crate::definitions::providers::terminal_clearer::Provide;
use crate::definitions::resolvers::terminal_clearer::Resolve;
use crate::definitions::use_cases::terminal_clearer::TerminalClearError;
use crate::providers::terminal_clearer as provider;
use crate::resolvers::terminal_clearer as resolver;

pub fn clear() -> Result<(), TerminalClearError> {
    let resolve: Resolve = resolver::resolve;
    let provide: Provide = provider::provide;

    clear_with(resolve, provide)
}

pub(crate) fn clear_with(resolve: Resolve, provide: Provide) -> Result<(), TerminalClearError> {
    resolve(provide)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::use_cases::terminal_clearer::TerminalClearer;

    #[test]
    fn terminal_clearer_clear_matches_use_case_function_pointer() {
        let clear_fn: TerminalClearer = clear;

        let _ = clear_fn;
    }

    #[test]
    fn terminal_clearer_agent_delegates_to_resolver() {
        fn resolve(provide: Provide) -> Result<(), TerminalClearError> {
            provide()
        }

        fn provide() -> Result<(), TerminalClearError> {
            Ok(())
        }

        let result = clear_with(resolve, provide);
        assert!(result.is_ok());
    }
}
