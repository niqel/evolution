use crate::collaborators::divider as collaborator;
use crate::definitions::types::number::Number;
use crate::definitions::use_cases::divide as divide_use_case;

pub fn divide(left: Number, right: Number) -> Result<Number, divide_use_case::Error> {
    collaborator::collaborate(left, right)
}
