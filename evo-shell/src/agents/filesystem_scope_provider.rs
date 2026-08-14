use crate::definitions::contracts::provide_filesystem_scope;
use crate::definitions::requesters::scope_requester;
use crate::definitions::use_cases::filesystem_scope;
use crate::resolvers::filesystem_scope_resolver;

pub fn provide(
    source: &str,
    request: scope_requester::Request,
    provide: provide_filesystem_scope::Provide,
) -> Result<(), filesystem_scope::Error> {
    filesystem_scope_resolver::resolve(provide, source, request)
}

pub const PROVIDE: filesystem_scope::Provide = provide;
