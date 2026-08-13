use crate::definitions::use_cases::rename;

pub type Request = fn(Result<(), rename::Error>);
