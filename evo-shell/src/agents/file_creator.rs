use crate::definitions::contracts::create_file;
use crate::definitions::requesters::create_file_requester;
use crate::definitions::use_cases::create_file as create_file_use_case;
use crate::resolvers::create_file_resolver;

pub fn create_file(
    target: &str,
    request: create_file_requester::Request,
    create: create_file::CreateFile,
) {
    create_file_resolver::resolve(create, target, request);
}

pub const CREATE: create_file_use_case::CreateFile = create_file;
