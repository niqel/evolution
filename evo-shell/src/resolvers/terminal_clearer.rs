use crate::definitions::providers::terminal_clearer::Provide;
use crate::definitions::use_cases::terminal_clearer::TerminalClearError;

pub fn resolve(provide: Provide) -> Result<(), TerminalClearError> {
    provide()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::definitions::resolvers::terminal_clearer::Resolve;

    #[test]
    fn terminal_clearer_resolver_delegates_to_provider() {
        fn provide() -> Result<(), TerminalClearError> {
            Ok(())
        }

        let resolve_fn: Resolve = resolve;

        let result = resolve_fn(provide);
        assert!(result.is_ok());
    }
}
