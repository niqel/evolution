use crate::definitions::contracts::activate_scope as activate_scope_contract;
use crate::definitions::structs::borrowed::scope::Scope;
use crate::definitions::use_cases::activate_scope;
use crate::resolvers::activate_scope_resolver;

pub fn activate(
    scope: Scope<'_>,
    activate: activate_scope_contract::Activate,
) -> Result<(), activate_scope::Error> {
    activate_scope_resolver::resolve(activate, scope)
}

pub const ACTIVATE: activate_scope::Activate = activate;
