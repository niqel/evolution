#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    TrashUnavailable,
}

pub type Trash = for<'target> fn(&'target str) -> Result<(), Error>;
