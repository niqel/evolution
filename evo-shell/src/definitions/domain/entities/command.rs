use crate::definitions::domain::value_objects::terminal_clear_mode::TerminalClearMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command<'a> {
    ScopeFs(&'a str),
    Iter,
    Enter(&'a str),
    Clear(TerminalClearMode),
    Exit,
}
