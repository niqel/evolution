use crate::definitions::contracts::provide_scope;
use crate::definitions::requesters::scope_requester;
use crate::definitions::use_cases::respond_scope;
use crate::resolvers::scope_resolver;

pub fn respond(
    provide: provide_scope::Provide,
    request: scope_requester::Request,
) -> Result<(), respond_scope::Error> {
    scope_resolver::resolve(provide, request)
}
