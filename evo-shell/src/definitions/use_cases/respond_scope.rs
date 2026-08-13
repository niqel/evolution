use crate::definitions::contracts::provide_scope;
use crate::definitions::requesters::scope_requester;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    ScopeUnavailable,
}

pub type Respond = fn(scope_requester::Request, provide_scope::Provide) -> Result<(), Error>;
