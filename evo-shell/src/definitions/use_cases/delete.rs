use crate::definitions::contracts::delete as delete_contract;
use crate::definitions::requesters::delete_requester;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    DeleteUnavailable,
}

pub type Delete = for<'target> fn(&'target str, delete_requester::Request, delete_contract::Delete);
