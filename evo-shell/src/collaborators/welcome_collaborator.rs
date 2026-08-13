use crate::definitions::requesters::welcome_requester;
use crate::definitions::structs::borrowed::welcome_information::WelcomeInformation;
use crate::tools::shell_information;

const COMPANY: &str = "CatarinaSoft";
const MESSAGE: &str = "Evo shell is a life :)";

pub fn collaborate(request: welcome_requester::Request) {
    let welcome = WelcomeInformation {
        company: COMPANY,
        shell: shell_information::get(),
        message: MESSAGE,
    };

    request(welcome);
}
