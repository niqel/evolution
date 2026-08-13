use crate::definitions::contracts::create_dir;
use crate::definitions::requesters::create_dir_requester;
use crate::definitions::use_cases::create_dir as create_dir_use_case;
use crate::resolvers::create_dir_resolver;

pub fn create(target: &str, request: create_dir_requester::Request, create: create_dir::CreateDir) {
    create_dir_resolver::resolve(create, target, request);
}

pub const CREATE: create_dir_use_case::Create = create;
