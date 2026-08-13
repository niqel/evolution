use crate::definitions::structs::borrowed::shell_information::ShellInformation;
use crate::tools::shell_information;

pub fn collaborate() -> ShellInformation<'static> {
    shell_information::get()
}
