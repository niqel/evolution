use crate::definitions::use_cases::create_file;

pub type Request = fn(Result<(), create_file::Error>);
