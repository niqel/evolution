use crate::definitions::requesters::filesystem_item_requester;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub type Enumerate =
    for<'source> fn(&'source str, filesystem_item_requester::Request) -> Result<(), Error>;
