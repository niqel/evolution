#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Scope<'scope> {
    pub scope_type: &'scope str,
    pub server: &'scope str,
    pub user: &'scope str,
    pub source: &'scope str,
    pub item: Option<&'scope str>,
}
