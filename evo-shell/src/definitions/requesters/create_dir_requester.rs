use crate::definitions::use_cases::create_dir;

pub type Request = fn(Result<(), create_dir::Error>);
