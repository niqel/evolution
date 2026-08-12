use crate::definitions::structs::borrowed::shell_information::ShellInformation;

pub type Inform = fn() -> ShellInformation<'static>;
