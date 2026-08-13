use crate::definitions::contracts::rename as rename_contract;
use crate::definitions::requesters::rename_requester;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    RenameUnavailable,
}

pub type Rename = for<'target, 'new_name> fn(
    &'target str,
    &'new_name str,
    rename_requester::Request,
    rename_contract::Rename,
);
