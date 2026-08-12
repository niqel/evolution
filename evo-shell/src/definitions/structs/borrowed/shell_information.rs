#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShellInformation<'shell> {
    pub name: &'shell str,
    pub version: &'shell str,
    pub description: &'shell str,
}
