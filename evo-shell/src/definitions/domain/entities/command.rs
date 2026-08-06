use crate::definitions::domain::value_objects::pipeline::Pipeline;
use crate::definitions::domain::value_objects::terminal_clear_mode::TerminalClearMode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandArgument<'a> {
    Literal(&'a str),
    Grouped(Box<Command<'a>>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command<'a> {
    ScopeFs(&'a str),
    Iter,
    Enter(CommandArgument<'a>),
    Clear(TerminalClearMode),
    Exit,
    Pipeline(Pipeline),
    Grouped(Box<Command<'a>>),
}
