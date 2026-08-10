use crate::definitions::contracts::provide_scope;
use crate::definitions::contracts::write_terminal;
use crate::definitions::use_cases::present_full_scope;
use crate::handlers::full_scope_handler;
use crate::resolvers::scope_resolver;

pub fn present(
    provide: provide_scope::Provide,
    write: write_terminal::Write,
) -> Result<(), present_full_scope::Error> {
    let continuation = full_scope_handler::handle;

    match scope_resolver::resolve(provide, continuation, write) {
        Ok(()) => Ok(()),
        Err(scope_resolver::Error::Scope(_)) => Err(present_full_scope::Error::ScopeUnavailable),
        Err(scope_resolver::Error::Terminal(_)) => {
            Err(present_full_scope::Error::TerminalUnavailable)
        }
    }
}
