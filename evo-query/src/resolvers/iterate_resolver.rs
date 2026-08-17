use crate::definitions::contracts::iterate as iterate_contract;
use crate::definitions::requesters::construction_requester;
use crate::definitions::structs::borrowed::iteration::Iteration;
use crate::definitions::use_cases::iterate;

pub fn resolve<'iteration>(
    contract: iterate_contract::Iterate,
    iteration: Iteration<'iteration>,
    request: construction_requester::Request,
) -> Result<(), iterate::Error<'iteration>> {
    contract(iteration, request).map_err(|error| match error {
        iterate_contract::Error::Unavailable => iterate::Error::IterationUnavailable,
        iterate_contract::Error::FieldNotFound(field) => iterate::Error::FieldNotFound(field),
        iterate_contract::Error::ComparisonTypeMismatch(field) => {
            iterate::Error::ComparisonTypeMismatch(field)
        }
        iterate_contract::Error::ExternalTypeIncompatible(field) => {
            iterate::Error::ExternalTypeIncompatible(field)
        }
        iterate_contract::Error::ProviderIncompatible => iterate::Error::ProviderIncompatible,
        iterate_contract::Error::ToValueRequiresSingleField => {
            iterate::Error::ToValueRequiresSingleField
        }
        iterate_contract::Error::ToValueRequiresRecord => iterate::Error::ToValueRequiresRecord,
        iterate_contract::Error::TextExpected => iterate::Error::TextExpected,
        iterate_contract::Error::UnsignedExpected => iterate::Error::UnsignedExpected,
        iterate_contract::Error::SubstringOutOfBounds => iterate::Error::SubstringOutOfBounds,
        iterate_contract::Error::ReplaceEmptyPattern => iterate::Error::ReplaceEmptyPattern,
    })
}
