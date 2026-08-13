use crate::definitions::use_cases::move_to;

pub type Request = fn(Result<(), move_to::Error>);
