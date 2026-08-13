use crate::definitions::contracts::enumerate_filesystem as enumerate_filesystem_contract;
use crate::definitions::requesters::enumerate_filesystem_requester;
use crate::definitions::requesters::filesystem_item_requester;
use crate::definitions::structs::borrowed::scope::Scope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    EnumerationUnavailable,
}

pub type Enumerate = for<'scope> fn(
    Scope<'scope>,
    filesystem_item_requester::Request,
    enumerate_filesystem_requester::Request,
    enumerate_filesystem_contract::Enumerate,
);
