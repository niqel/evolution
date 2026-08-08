#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub type Write = for<'content> fn(&'content str) -> Result<(), Error>;
