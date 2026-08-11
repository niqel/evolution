#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub type Copy = for<'origin, 'destination> fn(&'origin str, &'destination str) -> Result<(), Error>;
