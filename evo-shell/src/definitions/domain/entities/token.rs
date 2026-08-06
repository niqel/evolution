#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'a> {
    Word(&'a str),
    String(&'a str),
    PipelineSeparator,
    Comma,
    LeftParen,
    RightParen,
    Colon,
}
