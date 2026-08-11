use crate::collaborators::arithmetic;
use crate::definitions::types::number::Number;
use crate::definitions::use_cases::multiply as multiply_use_case;

pub fn multiply(left: Number, right: Number) -> Result<Number, multiply_use_case::Error> {
    arithmetic::multiply(left, right)
}
