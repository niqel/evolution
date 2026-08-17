#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    OutOfBounds,
}

pub type Substring = for<'text> fn(&'text str, usize, usize) -> Result<&'text str, Error>;
