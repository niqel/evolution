use crate::callbacks::full_scope_consumer;
use crate::definitions::contracts::provide_scope;
use crate::definitions::contracts::write_terminal;
use crate::definitions::use_cases::present_full_scope;
use crate::resolvers::scope_resolver;

pub fn present(
    provide: provide_scope::Provide,
    write: write_terminal::Write,
) -> Result<(), present_full_scope::Error> {
    let consume = full_scope_consumer::consume;

    match scope_resolver::resolve(provide, consume, write) {
        Ok(()) => Ok(()),
        Err(scope_resolver::Error::Scope(_)) => Err(present_full_scope::Error::ScopeUnavailable),
        Err(scope_resolver::Error::Terminal(_)) => {
            Err(present_full_scope::Error::TerminalUnavailable)
        }
    }
}
