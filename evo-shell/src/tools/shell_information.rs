use crate::definitions::structs::borrowed::shell_information::ShellInformation;

const NAME: &str = "Evolution Shell";
const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = "A lightweight functional shell.";

pub fn get() -> ShellInformation<'static> {
    ShellInformation {
        name: NAME,
        version: VERSION,
        description: DESCRIPTION,
    }
}
