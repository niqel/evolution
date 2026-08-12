use crate::collaborators::shell_informant;
use crate::definitions::structs::borrowed::shell_information::ShellInformation;

pub fn inform() -> ShellInformation<'static> {
    shell_informant::collaborate()
}
