use crate::definitions::use_cases::enumerate_filesystem;

pub type Request = fn(Result<(), enumerate_filesystem::Error>);
