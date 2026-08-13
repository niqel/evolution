use crate::collaborators::shell_informant;
use crate::definitions::requesters::shell_information_requester;
use crate::definitions::use_cases::inform_shell;

pub fn inform(request: shell_information_requester::Request) {
    shell_informant::collaborate(request);
}

pub const INFORM: inform_shell::Inform = inform;
