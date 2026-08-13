use crate::collaborators::welcome_collaborator;
use crate::definitions::requesters::welcome_requester;
use crate::definitions::use_cases::respond_welcome;

pub fn respond(request: welcome_requester::Request) {
    welcome_collaborator::collaborate(request);
}

pub const RESPOND: respond_welcome::Respond = respond;
