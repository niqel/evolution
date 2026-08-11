use crate::collaborators::summator as collaborator;
use crate::definitions::types::number::Number;
use crate::definitions::use_cases::sum as sum_use_case;

pub fn sum(left: Number, right: Number) -> Result<Number, sum_use_case::Error> {
    collaborator::collaborate(left, right)
}
