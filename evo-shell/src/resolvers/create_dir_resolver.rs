use crate::definitions::contracts::create_dir;
use crate::definitions::requesters::create_dir_requester;
use crate::definitions::use_cases::create_dir as create_dir_use_case;

pub fn resolve(
    create: create_dir::CreateDir,
    target: &str,
    request: create_dir_requester::Request,
) {
    let result = create(target).map_err(|_| create_dir_use_case::Error::CreateDirUnavailable);
    request(result);
}
