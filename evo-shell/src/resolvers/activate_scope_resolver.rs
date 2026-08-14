use crate::definitions::contracts::activate_scope as activate_scope_contract;
use crate::definitions::structs::borrowed::scope::Scope;
use crate::definitions::use_cases::activate_scope;

pub fn resolve(
    activate: activate_scope_contract::Activate,
    scope: Scope<'_>,
) -> Result<(), activate_scope::Error> {
    activate(scope).map_err(|_| activate_scope::Error::ActivationUnavailable)
}
