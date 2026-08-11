use crate::definitions::contracts::provide_scope;
use crate::definitions::contracts::write_terminal;
use crate::definitions::use_cases::present_scope;
use crate::handlers::scope_handler;
use crate::resolvers::scope_resolver;

pub fn present(
    provide: provide_scope::Provide,
    write: write_terminal::Write,
) -> Result<(), present_scope::Error> {
    scope_resolver::resolve(provide, scope_handler::handle, write).map_err(|err| match err {
        scope_resolver::Error::Scope(_) => present_scope::Error::ScopeUnavailable,
        scope_resolver::Error::Terminal(_) => present_scope::Error::TerminalUnavailable,
    })
}
