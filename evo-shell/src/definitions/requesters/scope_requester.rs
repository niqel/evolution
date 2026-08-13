use crate::definitions::structs::borrowed::scope::Scope;

pub type Request = for<'scope> fn(Scope<'scope>);
