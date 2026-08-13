use crate::definitions::contracts::provide_scope;
use crate::definitions::requesters::scope_requester;
use crate::definitions::use_cases::respond_scope;
use crate::resolvers::scope_resolver;

pub fn respond(
    request: scope_requester::Request,
    provide: provide_scope::Provide,
) -> Result<(), respond_scope::Error> {
    scope_resolver::resolve(provide, request)
}

pub const RESPOND: respond_scope::Respond = respond;
