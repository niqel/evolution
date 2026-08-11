use crate::definitions::types::number::Number;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    UnsupportedTypes,
    Overflow,
    DivisionByZero,
}

pub type Divide = fn(left: Number, right: Number) -> Result<Number, Error>;
