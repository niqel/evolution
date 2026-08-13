use crate::definitions::contracts::create_dir;
use crate::definitions::requesters::create_dir_requester;
use crate::resolvers::create_dir_resolver;

pub fn create(create: create_dir::CreateDir, target: &str, request: create_dir_requester::Request) {
    create_dir_resolver::resolve(create, target, request);
}
