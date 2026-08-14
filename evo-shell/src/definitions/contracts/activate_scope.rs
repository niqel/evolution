use crate::definitions::structs::borrowed::scope::Scope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub type Activate = for<'scope> fn(Scope<'scope>) -> Result<(), Error>;
