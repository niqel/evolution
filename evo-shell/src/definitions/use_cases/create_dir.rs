use crate::definitions::requesters::create_dir_requester;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    CreateDirUnavailable,
}

pub type Create = for<'target> fn(&'target str, create_dir_requester::Request);
