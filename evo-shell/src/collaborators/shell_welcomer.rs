use crate::collaborators::shell_informant;
use crate::definitions::structs::borrowed::welcome_information::WelcomeInformation;

const COMPANY: &str = "CatarinaSoft";
const MESSAGE: &str = "Evo shell is a life :)";

pub fn collaborate() -> WelcomeInformation<'static> {
    WelcomeInformation {
        company: COMPANY,
        shell: shell_informant::collaborate(),
        message: MESSAGE,
    }
}
