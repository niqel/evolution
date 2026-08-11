use crate::definitions::types::number::Number;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NumberBinding<'binding> {
    pub name: &'binding str,
    pub value: Number,
}
