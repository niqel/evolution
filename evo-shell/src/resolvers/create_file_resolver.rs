use crate::definitions::contracts::create_file;
use crate::definitions::requesters::create_file_requester;
use crate::definitions::use_cases::create_file as create_file_use_case;

pub fn resolve(
    create: create_file::CreateFile,
    target: &str,
    request: create_file_requester::Request,
) {
    let result = create(target).map_err(|_| create_file_use_case::Error::CreateFileUnavailable);
    request(result);
}
