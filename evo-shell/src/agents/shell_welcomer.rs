use crate::collaborators::{
    shell_informant as shell_informant_collaborator, shell_welcomer as shell_welcomer_collaborator,
};
use crate::definitions::structs::borrowed::welcome_information::WelcomeInformation;

pub fn welcome() -> WelcomeInformation<'static> {
    let shell = shell_informant_collaborator::collaborate();
    shell_welcomer_collaborator::collaborate(shell)
}
