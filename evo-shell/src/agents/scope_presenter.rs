use crate::callbacks::scope_consumer;
use crate::definitions::contracts::provide_scope;
use crate::definitions::contracts::write_terminal;
use crate::definitions::use_cases::present_scope;
use crate::resolvers::scope_resolver;

pub fn present(
    provide: provide_scope::Provide,
    write: write_terminal::Write,
) -> Result<(), present_scope::Error> {
    let consume = scope_consumer::consume;

    match scope_resolver::resolve(provide, consume, write) {
        Ok(()) => Ok(()),
        Err(scope_resolver::Error::Scope(_)) => Err(present_scope::Error::ScopeUnavailable),
        Err(scope_resolver::Error::Terminal(_)) => Err(present_scope::Error::TerminalUnavailable),
    }
}
