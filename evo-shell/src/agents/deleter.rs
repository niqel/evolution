use crate::definitions::contracts::delete;
use crate::definitions::requesters::delete_requester;
use crate::definitions::use_cases::delete as delete_use_case;
use crate::resolvers::delete_resolver;

pub fn delete(target: &str, request: delete_requester::Request, delete_operation: delete::Delete) {
    delete_resolver::resolve(delete_operation, target, request);
}

pub const DELETE: delete_use_case::Delete = delete;
