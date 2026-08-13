use crate::collaborators::about_collaborator;
use crate::definitions::requesters::about_requester;

pub fn respond(request: about_requester::Request) {
    about_collaborator::collaborate(request);
}
