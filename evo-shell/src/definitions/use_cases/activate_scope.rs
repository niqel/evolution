use crate::definitions::contracts::activate_scope;
use crate::definitions::structs::borrowed::scope::Scope;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    ActivationUnavailable,
}

pub type Activate = for<'scope> fn(Scope<'scope>, activate_scope::Activate) -> Result<(), Error>;
