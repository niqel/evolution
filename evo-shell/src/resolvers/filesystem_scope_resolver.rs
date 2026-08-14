use crate::definitions::contracts::provide_filesystem_scope;
use crate::definitions::requesters::scope_requester;
use crate::definitions::use_cases::filesystem_scope;

pub fn resolve(
    provide: provide_filesystem_scope::Provide,
    source: &str,
    request: scope_requester::Request,
) -> Result<(), filesystem_scope::Error> {
    provide(source, request).map_err(|_| filesystem_scope::Error::ScopeUnavailable)
}
