use crate::definitions::structs::borrowed::shell_information::ShellInformation;

pub type Request = for<'information> fn(ShellInformation<'information>);
