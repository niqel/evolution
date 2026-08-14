use crate::definitions::contracts::enter_scope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    NavigationUnavailable,
}

pub type Enter = for<'target> fn(&'target str, enter_scope::Enter) -> Result<(), Error>;
