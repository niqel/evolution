#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub type Rename = for<'target, 'new_name> fn(&'target str, &'new_name str) -> Result<(), Error>;
