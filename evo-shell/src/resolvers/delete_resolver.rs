use crate::definitions::contracts::delete;
use crate::definitions::requesters::delete_requester;
use crate::definitions::use_cases::delete as delete_use_case;

pub fn resolve(delete_operation: delete::Delete, target: &str, request: delete_requester::Request) {
    let result = delete_operation(target).map_err(|_| delete_use_case::Error::DeleteUnavailable);
    request(result);
}
