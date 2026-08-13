use crate::definitions::use_cases::trash;

pub type Request = fn(Result<(), trash::Error>);
