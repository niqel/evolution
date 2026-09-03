use evo_query::definitions::contracts::iterate as iterate_contract;
use evo_query::definitions::requesters::construction_requester;
use evo_query::definitions::structs::borrowed::construction::Construction;
use evo_query::definitions::structs::borrowed::iteration::Iteration;
use evo_query::definitions::structs::borrowed::iteration_operation::IterationOperation;
use evo_query::definitions::structs::borrowed::selection::Selection;
use evo_query::definitions::structs::owned::flow::Flow;
use evo_query::definitions::use_cases::iterate as iterate_use_case;
use evo_values::definitions::value::Value;

fn receive(_construction: Construction<'_>) -> Flow {
    Flow::Continue
}

fn contract_success<'iteration>(
    iteration: Iteration<'iteration>,
    request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    assert_eq!(iteration.operations.len(), 1);
    assert_eq!(iteration.operations[0], IterationOperation::Take(1));

    let flow = request(Construction::Value(Value::Uint64(42)));
    assert_eq!(flow, Flow::Continue);

    Ok(())
}

fn contract_unavailable<'iteration>(
    _iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    Err(iterate_contract::Error::Unavailable)
}

fn contract_field_not_found<'iteration>(
    _iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    Err(iterate_contract::Error::FieldNotFound("missing"))
}

fn contract_comparison_type_mismatch<'iteration>(
    _iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    Err(iterate_contract::Error::ComparisonTypeMismatch("size"))
}

fn contract_external_type_incompatible<'iteration>(
    _iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    Err(iterate_contract::Error::ExternalTypeIncompatible("created"))
}

fn contract_provider_incompatible<'iteration>(
    _iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    Err(iterate_contract::Error::ProviderIncompatible)
}

fn contract_to_value_requires_single_field<'iteration>(
    _iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    Err(iterate_contract::Error::ToValueRequiresSingleField)
}

fn contract_to_value_requires_record<'iteration>(
    _iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    Err(iterate_contract::Error::ToValueRequiresRecord)
}

fn fake_use_case<'iteration>(
    iteration: Iteration<'iteration>,
    request: construction_requester::Request,
    contract: iterate_contract::Iterate,
) -> Result<(), iterate_use_case::Error<'iteration>> {
    match contract(iteration, request) {
        Ok(()) => Ok(()),
        Err(iterate_contract::Error::Unavailable) => {
            Err(iterate_use_case::Error::IterationUnavailable)
        }
        Err(iterate_contract::Error::FieldNotFound(field)) => {
            Err(iterate_use_case::Error::FieldNotFound(field))
        }
        Err(iterate_contract::Error::ComparisonTypeMismatch(field)) => {
            Err(iterate_use_case::Error::ComparisonTypeMismatch(field))
        }
        Err(iterate_contract::Error::ExternalTypeIncompatible(field)) => {
            Err(iterate_use_case::Error::ExternalTypeIncompatible(field))
        }
        Err(iterate_contract::Error::ProviderIncompatible) => {
            Err(iterate_use_case::Error::ProviderIncompatible)
        }
        Err(iterate_contract::Error::ToValueRequiresSingleField) => {
            Err(iterate_use_case::Error::ToValueRequiresSingleField)
        }
        Err(iterate_contract::Error::ToValueRequiresRecord) => {
            Err(iterate_use_case::Error::ToValueRequiresRecord)
        }
        Err(iterate_contract::Error::TextExpected) => Err(iterate_use_case::Error::TextExpected),
        Err(iterate_contract::Error::UnsignedExpected) => {
            Err(iterate_use_case::Error::UnsignedExpected)
        }
        Err(iterate_contract::Error::SubstringOutOfBounds) => {
            Err(iterate_use_case::Error::SubstringOutOfBounds)
        }
        Err(iterate_contract::Error::ReplaceEmptyPattern) => {
            Err(iterate_use_case::Error::ReplaceEmptyPattern)
        }
    }
}

#[test]
fn iterate_use_case_signature_and_success() {
    let operations = [IterationOperation::Take(1)];

    let iteration = Iteration {
        operations: &operations,
    };

    let use_case: iterate_use_case::Iterate = fake_use_case;

    let result = use_case(iteration, receive, contract_success);
    assert_eq!(result, Ok(()));
}

#[test]
fn iterate_use_case_error() {
    let operations: [IterationOperation<'_>; 0] = [];

    let iteration = Iteration {
        operations: &operations,
    };

    let use_case: iterate_use_case::Iterate = fake_use_case;

    let result = use_case(iteration, receive, contract_unavailable);
    assert_eq!(result, Err(iterate_use_case::Error::IterationUnavailable));
}

#[test]
fn iterate_use_case_field_not_found_error() {
    let selections = [Selection::Field("name"), Selection::Field("missing")];
    let operations = [IterationOperation::Select(&selections)];

    let iteration = Iteration {
        operations: &operations,
    };

    let use_case: iterate_use_case::Iterate = fake_use_case;

    let result = use_case(iteration, receive, contract_field_not_found);
    assert_eq!(
        result,
        Err(iterate_use_case::Error::FieldNotFound("missing"))
    );
}

#[test]
fn iterate_use_case_comparison_type_mismatch_error() {
    let operations: [IterationOperation<'_>; 0] = [];

    let iteration = Iteration {
        operations: &operations,
    };

    let use_case: iterate_use_case::Iterate = fake_use_case;

    let result = use_case(iteration, receive, contract_comparison_type_mismatch);
    assert_eq!(
        result,
        Err(iterate_use_case::Error::ComparisonTypeMismatch("size"))
    );
}

#[test]
fn iterate_use_case_external_type_incompatible_error() {
    let operations: [IterationOperation<'_>; 0] = [];

    let iteration = Iteration {
        operations: &operations,
    };

    let use_case: iterate_use_case::Iterate = fake_use_case;

    let result = use_case(iteration, receive, contract_external_type_incompatible);
    assert_eq!(
        result,
        Err(iterate_use_case::Error::ExternalTypeIncompatible("created"))
    );
}

#[test]
fn iterate_use_case_provider_incompatible_error() {
    let operations: [IterationOperation<'_>; 0] = [];

    let iteration = Iteration {
        operations: &operations,
    };

    let use_case: iterate_use_case::Iterate = fake_use_case;

    let result = use_case(iteration, receive, contract_provider_incompatible);
    assert_eq!(result, Err(iterate_use_case::Error::ProviderIncompatible));
}

#[test]
fn iterate_use_case_to_value_requires_single_field_error() {
    let operations = [IterationOperation::ToValue];

    let iteration = Iteration {
        operations: &operations,
    };

    let use_case: iterate_use_case::Iterate = fake_use_case;

    let result = use_case(iteration, receive, contract_to_value_requires_single_field);
    assert_eq!(
        result,
        Err(iterate_use_case::Error::ToValueRequiresSingleField)
    );
}

#[test]
fn iterate_use_case_to_value_requires_record_error() {
    let operations = [IterationOperation::ToValue];

    let iteration = Iteration {
        operations: &operations,
    };

    let use_case: iterate_use_case::Iterate = fake_use_case;

    let result = use_case(iteration, receive, contract_to_value_requires_record);
    assert_eq!(result, Err(iterate_use_case::Error::ToValueRequiresRecord));
}

#[test]
fn iterate_use_case_error_equality() {
    assert_eq!(
        iterate_use_case::Error::IterationUnavailable,
        iterate_use_case::Error::IterationUnavailable
    );
    assert_eq!(
        iterate_use_case::Error::FieldNotFound("name"),
        iterate_use_case::Error::FieldNotFound("name")
    );
    assert_eq!(
        iterate_use_case::Error::ComparisonTypeMismatch("size"),
        iterate_use_case::Error::ComparisonTypeMismatch("size")
    );
    assert_eq!(
        iterate_use_case::Error::ExternalTypeIncompatible("created"),
        iterate_use_case::Error::ExternalTypeIncompatible("created")
    );
    assert_eq!(
        iterate_use_case::Error::ProviderIncompatible,
        iterate_use_case::Error::ProviderIncompatible
    );
    assert_eq!(
        iterate_use_case::Error::ToValueRequiresSingleField,
        iterate_use_case::Error::ToValueRequiresSingleField
    );
    assert_eq!(
        iterate_use_case::Error::ToValueRequiresRecord,
        iterate_use_case::Error::ToValueRequiresRecord
    );

    assert_ne!(
        iterate_use_case::Error::FieldNotFound("name"),
        iterate_use_case::Error::FieldNotFound("size")
    );
    assert_ne!(
        iterate_use_case::Error::ComparisonTypeMismatch("size"),
        iterate_use_case::Error::ComparisonTypeMismatch("created")
    );
    assert_ne!(
        iterate_use_case::Error::ExternalTypeIncompatible("created"),
        iterate_use_case::Error::ExternalTypeIncompatible("modified")
    );
}
