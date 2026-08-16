use crate::definitions::contracts::iterate as iterate_contract;
use crate::definitions::requesters::construction_requester;
use crate::definitions::structs::borrowed::iteration::Iteration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    IterationUnavailable,
    ToValueRequiresSingleField,
    ToValueRequiresRecord,
}

pub type Iterate = for<'iteration> fn(
    Iteration<'iteration>,
    construction_requester::Request,
    iterate_contract::Iterate,
) -> Result<(), Error>;
