use crate::collaborators::arithmetic;
use crate::definitions::types::number::Number;
use crate::definitions::use_cases::subtract as subtract_use_case;

pub fn subtract(left: Number, right: Number) -> Result<Number, subtract_use_case::Error> {
    arithmetic::subtract(left, right)
}
