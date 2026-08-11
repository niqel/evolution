use crate::definitions::types::number::Number;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    UnsupportedTypes,
    Overflow,
}

pub type Subtract = fn(left: Number, right: Number) -> Result<Number, Error>;
