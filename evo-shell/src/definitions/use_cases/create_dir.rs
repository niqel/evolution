#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    CreateDirUnavailable,
}

pub type CreateDir = for<'target> fn(&'target str) -> Result<(), Error>;
