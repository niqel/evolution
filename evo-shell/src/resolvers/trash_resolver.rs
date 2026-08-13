use crate::definitions::contracts::trash;
use crate::definitions::requesters::trash_requester;
use crate::definitions::use_cases::trash as trash_use_case;

pub fn resolve(trash_operation: trash::Trash, target: &str, request: trash_requester::Request) {
    let result = trash_operation(target).map_err(|_| trash_use_case::Error::TrashUnavailable);
    request(result);
}
