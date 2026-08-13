use crate::definitions::contracts::provide_scope;
use crate::definitions::requesters::scope_requester;
use crate::definitions::use_cases::respond_scope;

pub fn resolve(
    provide: provide_scope::Provide,
    request: scope_requester::Request,
) -> Result<(), respond_scope::Error> {
    provide(request).map_err(|_| respond_scope::Error::ScopeUnavailable)
}
