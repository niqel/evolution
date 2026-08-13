use crate::definitions::contracts::create_file as create_file_contract;
use crate::definitions::requesters::create_file_requester;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    CreateFileUnavailable,
}

pub type CreateFile =
    for<'target> fn(&'target str, create_file_requester::Request, create_file_contract::CreateFile);
