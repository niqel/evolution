use crate::definitions::structs::borrowed::shell_information::ShellInformation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WelcomeInformation<'welcome> {
    pub company: &'welcome str,
    pub shell: ShellInformation<'welcome>,
    pub message: &'welcome str,
}
