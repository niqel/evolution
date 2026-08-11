use crate::definitions::contracts::provide_scope;
use crate::definitions::contracts::write_terminal;
use crate::definitions::use_cases::present_full_scope;
use crate::handlers::full_scope_handler;
use crate::resolvers::scope_resolver;

pub fn present(
    provide: provide_scope::Provide,
    write: write_terminal::Write,
) -> Result<(), present_full_scope::Error> {
    scope_resolver::resolve(provide, full_scope_handler::handle, write).map_err(|err| match err {
        scope_resolver::Error::Scope(_) => present_full_scope::Error::ScopeUnavailable,
        scope_resolver::Error::Terminal(_) => present_full_scope::Error::TerminalUnavailable,
    })
}
