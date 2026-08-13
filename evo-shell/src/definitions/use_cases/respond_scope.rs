use crate::definitions::requesters::scope_requester;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    ScopeUnavailable,
}

pub type Respond = fn(scope_requester::Request) -> Result<(), Error>;
