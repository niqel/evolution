use crate::definitions::contracts::provide_filesystem_scope;
use crate::definitions::requesters::scope_requester;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    ScopeUnavailable,
}

pub type Provide = for<'source> fn(
    &'source str,
    scope_requester::Request,
    provide_filesystem_scope::Provide,
) -> Result<(), Error>;
