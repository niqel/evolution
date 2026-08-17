use crate::definitions::requesters::construction_requester;
use crate::definitions::structs::borrowed::iteration::Iteration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error<'error> {
    Unavailable,
    FieldNotFound(&'error str),
    ComparisonTypeMismatch(&'error str),
    ExternalTypeIncompatible(&'error str),
    ProviderIncompatible,
    ToValueRequiresSingleField,
    ToValueRequiresRecord,
    TextExpected,
    UnsignedExpected,
    SubstringOutOfBounds,
    ReplaceEmptyPattern,
}

pub type Iterate = for<'iteration> fn(
    Iteration<'iteration>,
    construction_requester::Request,
) -> Result<(), Error<'iteration>>;
