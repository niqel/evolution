use crate::definitions::requesters::about_requester;
use crate::tools::shell_information;

pub fn collaborate(request: about_requester::Request) {
    let about = shell_information::get();
    request(about);
}
