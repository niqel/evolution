#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    CreateFileUnavailable,
}

pub type CreateFile = for<'target> fn(&'target str) -> Result<(), Error>;
