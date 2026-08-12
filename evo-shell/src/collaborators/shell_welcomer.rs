use crate::definitions::structs::borrowed::shell_information::ShellInformation;
use crate::definitions::structs::borrowed::welcome_information::WelcomeInformation;

const COMPANY: &str = "CatarinaSoft";
const MESSAGE: &str = "Evo shell is a life :)";

pub fn collaborate<'shell>(shell: ShellInformation<'shell>) -> WelcomeInformation<'shell> {
    WelcomeInformation {
        company: COMPANY,
        shell,
        message: MESSAGE,
    }
}
