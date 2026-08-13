use crate::definitions::contracts::provide_scope;
use crate::definitions::requesters::scope_requester;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Scope(provide_scope::Error),
}

pub fn resolve(
    provide: provide_scope::Provide,
    request: scope_requester::Request,
) -> Result<(), Error> {
    provide(request).map_err(Error::Scope)
}
