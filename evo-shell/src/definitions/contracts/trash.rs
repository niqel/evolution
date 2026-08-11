#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub type Trash = for<'target> fn(&'target str) -> Result<(), Error>;
