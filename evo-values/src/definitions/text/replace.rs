use alloc::string::String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    EmptyPattern,
}

pub type Replace = fn(text: &str, from: &str, to: &str) -> Result<String, Error>;
