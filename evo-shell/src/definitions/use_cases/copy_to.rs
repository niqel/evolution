#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    CopyUnavailable,
}

pub type Copy = for<'origin, 'destination> fn(&'origin str, &'destination str) -> Result<(), Error>;
