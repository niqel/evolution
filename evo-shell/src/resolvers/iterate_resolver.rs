use crate::definitions::contracts::iterate as iterate_contract;
use crate::definitions::requesters::construction_requester;
use crate::definitions::structs::borrowed::iteration::Iteration;
use crate::definitions::use_cases::iterate;

pub fn resolve(
    contract: iterate_contract::Iterate,
    iteration: Iteration<'_>,
    request: construction_requester::Request,
) -> Result<(), iterate::Error> {
    contract(iteration, request).map_err(|_| iterate::Error::IterationUnavailable)
}
