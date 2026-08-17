use crate::definitions::contracts::iterate as iterate_contract;
use crate::definitions::requesters::construction_requester;
use crate::definitions::structs::borrowed::iteration::Iteration;
use crate::definitions::use_cases::iterate as iterate_use_case;
use crate::resolvers::iterate_resolver;

pub fn iterate<'iteration>(
    iteration: Iteration<'iteration>,
    request: construction_requester::Request,
    contract: iterate_contract::Iterate,
) -> Result<(), iterate_use_case::Error<'iteration>> {
    iterate_resolver::resolve(contract, iteration, request)
}

pub const ITERATE: iterate_use_case::Iterate = iterate;
