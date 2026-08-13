use crate::definitions::contracts::trash as trash_contract;
use crate::definitions::requesters::trash_requester;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    TrashUnavailable,
}

pub type Trash = for<'target> fn(&'target str, trash_requester::Request, trash_contract::Trash);
