use crate::definitions::use_cases::copy_to;

pub type Request = fn(Result<(), copy_to::Error>);
