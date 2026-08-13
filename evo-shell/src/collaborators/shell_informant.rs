use crate::definitions::requesters::shell_information_requester;
use crate::tools::shell_information;

pub fn collaborate(request: shell_information_requester::Request) {
    let information = shell_information::get();
    request(information);
}
