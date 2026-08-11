use crate::collaborators::arithmetic;
use crate::definitions::types::number::Number;

pub type Error = arithmetic::Error;

pub type Divide = fn(left: Number, right: Number) -> Result<Number, Error>;
