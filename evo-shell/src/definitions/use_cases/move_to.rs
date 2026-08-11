#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    MoveUnavailable,
}

pub type Move = for<'origin, 'destination> fn(&'origin str, &'destination str) -> Result<(), Error>;
