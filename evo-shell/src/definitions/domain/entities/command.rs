#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command<'a> {
    ScopeFs(&'a str),
    Iter,
    Enter(&'a str),
}
