use crate::definitions::contracts::enumerate_filesystem as enumerate_filesystem_contract;
use crate::definitions::requesters::enumerate_filesystem_requester;
use crate::definitions::requesters::filesystem_item_requester;
use crate::definitions::structs::borrowed::scope::Scope;
use crate::definitions::use_cases::enumerate_filesystem;
use crate::resolvers::enumerate_filesystem_resolver;

pub fn enumerate(
    scope: Scope<'_>,
    item_request: filesystem_item_requester::Request,
    result_request: enumerate_filesystem_requester::Request,
    enumerate: enumerate_filesystem_contract::Enumerate,
) {
    enumerate_filesystem_resolver::resolve(enumerate, scope, item_request, result_request);
}

pub const ENUMERATE: enumerate_filesystem::Enumerate = enumerate;
