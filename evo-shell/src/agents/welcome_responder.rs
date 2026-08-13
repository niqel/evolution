use crate::collaborators::welcome_collaborator;
use crate::definitions::requesters::welcome_requester;

pub fn respond(request: welcome_requester::Request) {
    welcome_collaborator::collaborate(request);
}
