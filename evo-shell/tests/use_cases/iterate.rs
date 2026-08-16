use evo_shell::definitions::contracts::iterate as iterate_contract;
use evo_shell::definitions::requesters::construction_requester;
use evo_shell::definitions::structs::borrowed::construction::Construction;
use evo_shell::definitions::structs::borrowed::iteration::Iteration;
use evo_shell::definitions::structs::borrowed::iteration_operation::IterationOperation;
use evo_shell::definitions::structs::borrowed::value::Value;
use evo_shell::definitions::structs::owned::flow::Flow;
use evo_shell::definitions::use_cases::iterate as iterate_use_case;

fn receive(_construction: Construction<'_>) -> Flow {
    Flow::Continue
}

fn contract_success(
    iteration: Iteration<'_>,
    request: construction_requester::Request,
) -> Result<(), iterate_contract::Error> {
    assert_eq!(iteration.operations.len(), 2);
    assert_eq!(iteration.operations[0], IterationOperation::Take(1));
    assert_eq!(iteration.operations[1], IterationOperation::Iter);

    let flow = request(Construction::Value(Value::Unsigned(42)));
    assert_eq!(flow, Flow::Continue);

    Ok(())
}

fn contract_unavailable(
    _iteration: Iteration<'_>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error> {
    Err(iterate_contract::Error::Unavailable)
}

fn contract_to_value_requires_single_field(
    _iteration: Iteration<'_>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error> {
    Err(iterate_contract::Error::ToValueRequiresSingleField)
}

fn contract_to_value_requires_record(
    _iteration: Iteration<'_>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error> {
    Err(iterate_contract::Error::ToValueRequiresRecord)
}

fn fake_use_case(
    iteration: Iteration<'_>,
    request: construction_requester::Request,
    contract: iterate_contract::Iterate,
) -> Result<(), iterate_use_case::Error> {
    match contract(iteration, request) {
        Ok(()) => Ok(()),
        Err(iterate_contract::Error::Unavailable) => {
            Err(iterate_use_case::Error::IterationUnavailable)
        }
        Err(iterate_contract::Error::ToValueRequiresSingleField) => {
            Err(iterate_use_case::Error::ToValueRequiresSingleField)
        }
        Err(iterate_contract::Error::ToValueRequiresRecord) => {
            Err(iterate_use_case::Error::ToValueRequiresRecord)
        }
    }
}

#[test]
fn iterate_use_case_signature_and_success() {
    let operations = [IterationOperation::Take(1), IterationOperation::Iter];

    let iteration = Iteration {
        operations: &operations,
    };

    let use_case: iterate_use_case::Iterate = fake_use_case;

    let result = use_case(iteration, receive, contract_success);
    assert_eq!(result, Ok(()));
}

#[test]
fn iterate_use_case_error() {
    let operations = [IterationOperation::Iter];

    let iteration = Iteration {
        operations: &operations,
    };

    let use_case: iterate_use_case::Iterate = fake_use_case;

    let result = use_case(iteration, receive, contract_unavailable);
    assert_eq!(result, Err(iterate_use_case::Error::IterationUnavailable));
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
        iterate_use_case::Error::ToValueRequiresSingleField,
        iterate_use_case::Error::ToValueRequiresSingleField
    );
    assert_eq!(
        iterate_use_case::Error::ToValueRequiresRecord,
        iterate_use_case::Error::ToValueRequiresRecord
    );
}
