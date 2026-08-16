use crate::definitions::requesters::construction_requester;
use crate::definitions::structs::borrowed::iteration::Iteration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Unavailable,
}

pub type Iterate =
    for<'iteration> fn(Iteration<'iteration>, construction_requester::Request) -> Result<(), Error>;
