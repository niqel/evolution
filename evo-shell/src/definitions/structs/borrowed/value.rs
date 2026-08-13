#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value<'value> {
    Text(&'value str),
    Unsigned(u64),
    Signed(i64),
    Boolean(bool),
}
