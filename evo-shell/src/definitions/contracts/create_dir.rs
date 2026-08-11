#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub type CreateDir = for<'target> fn(&'target str) -> Result<(), Error>;
