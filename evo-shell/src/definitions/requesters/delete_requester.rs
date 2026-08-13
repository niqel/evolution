use crate::definitions::use_cases::delete;

pub type Request = fn(Result<(), delete::Error>);
