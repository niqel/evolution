use crate::definitions::contracts::trash;
use crate::definitions::requesters::trash_requester;
use crate::definitions::use_cases::trash as trash_use_case;
use crate::resolvers::trash_resolver;

pub fn trash(target: &str, request: trash_requester::Request, trash_operation: trash::Trash) {
    trash_resolver::resolve(trash_operation, target, request);
}

pub const TRASH: trash_use_case::Trash = trash;
