use crate::collaborators::about_collaborator;
use crate::definitions::requesters::about_requester;
use crate::definitions::use_cases::respond_about;

pub fn respond(request: about_requester::Request) {
    about_collaborator::collaborate(request);
}

pub const RESPOND: respond_about::Respond = respond;
