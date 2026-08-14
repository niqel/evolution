use crate::definitions::contracts::enter_scope as enter_scope_contract;
use crate::definitions::use_cases::enter_scope;
use crate::resolvers::enter_scope_resolver;

pub fn enter(target: &str, enter: enter_scope_contract::Enter) -> Result<(), enter_scope::Error> {
    enter_scope_resolver::resolve(enter, target)
}

pub const ENTER: enter_scope::Enter = enter;
