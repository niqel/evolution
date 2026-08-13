use crate::definitions::contracts::rename;
use crate::definitions::requesters::rename_requester;
use crate::definitions::use_cases::rename as rename_use_case;
use crate::resolvers::rename_resolver;

pub fn rename(
    target: &str,
    new_name: &str,
    request: rename_requester::Request,
    rename_operation: rename::Rename,
) {
    rename_resolver::resolve(rename_operation, target, new_name, request);
}

pub const RENAME: rename_use_case::Rename = rename;
