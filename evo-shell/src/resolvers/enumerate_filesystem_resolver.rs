use crate::definitions::contracts::enumerate_filesystem as enumerate_filesystem_contract;
use crate::definitions::requesters::enumerate_filesystem_requester;
use crate::definitions::requesters::filesystem_item_requester;
use crate::definitions::structs::borrowed::scope::Scope;
use crate::definitions::use_cases::enumerate_filesystem;

pub fn resolve(
    enumerate: enumerate_filesystem_contract::Enumerate,
    scope: Scope<'_>,
    item_request: filesystem_item_requester::Request,
    result_request: enumerate_filesystem_requester::Request,
) {
    let result = enumerate(scope.source, item_request)
        .map_err(|_| enumerate_filesystem::Error::EnumerationUnavailable);

    result_request(result);
}
