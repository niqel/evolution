use crate::collaborators::arithmetic;
use crate::definitions::types::number::Number;
use crate::definitions::use_cases::sum as sum_use_case;

pub fn sum(left: Number, right: Number) -> Result<Number, sum_use_case::Error> {
    arithmetic::sum(left, right)
}
