use crate::collaborators::arithmetic;
use crate::definitions::types::number::Number;

pub type Error = arithmetic::Error;

pub type Subtract = fn(left: Number, right: Number) -> Result<Number, Error>;
