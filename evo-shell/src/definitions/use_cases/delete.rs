#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    DeleteUnavailable,
}

pub type Delete = for<'target> fn(&'target str) -> Result<(), Error>;
