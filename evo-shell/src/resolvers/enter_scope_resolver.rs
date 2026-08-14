use crate::definitions::contracts::enter_scope as enter_scope_contract;
use crate::definitions::use_cases::enter_scope;

pub fn resolve(enter: enter_scope_contract::Enter, target: &str) -> Result<(), enter_scope::Error> {
    enter(target).map_err(|_| enter_scope::Error::NavigationUnavailable)
}
