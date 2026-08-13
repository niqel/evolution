use crate::definitions::contracts::rename;
use crate::definitions::requesters::rename_requester;
use crate::definitions::use_cases::rename as rename_use_case;

pub fn resolve(
    rename_operation: rename::Rename,
    target: &str,
    new_name: &str,
    request: rename_requester::Request,
) {
    let result =
        rename_operation(target, new_name).map_err(|_| rename_use_case::Error::RenameUnavailable);
    request(result);
}
