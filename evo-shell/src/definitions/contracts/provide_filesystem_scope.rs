use crate::definitions::requesters::scope_requester;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub type Provide = for<'source> fn(&'source str, scope_requester::Request) -> Result<(), Error>;
