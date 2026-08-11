#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    RenameUnavailable,
}

pub type Rename = for<'target, 'new_name> fn(&'target str, &'new_name str) -> Result<(), Error>;
