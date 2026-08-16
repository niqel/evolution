use evo_shell::definitions::contracts::iterate as iterate_contract;
use evo_shell::definitions::requesters::construction_requester;
use evo_shell::definitions::structs::borrowed::construction::Construction;
use evo_shell::definitions::structs::borrowed::iteration::Iteration;
use evo_shell::definitions::structs::borrowed::iteration_operation::IterationOperation;
use evo_shell::definitions::structs::borrowed::value::Value;
use evo_shell::definitions::structs::owned::flow::Flow;
use evo_shell::definitions::use_cases::iterate;
use evo_shell::resolvers::iterate_resolver;

fn receive(construction: Construction<'_>) -> Flow {
    assert_eq!(construction, Construction::Value(Value::Unsigned(42)));
    Flow::Continue
}

fn fake_contract_success(
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

fn fake_contract_unavailable(
    _iteration: Iteration<'_>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error> {
    Err(iterate_contract::Error::Unavailable)
}

fn fake_contract_to_value_requires_single_field(
    _iteration: Iteration<'_>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error> {
    Err(iterate_contract::Error::ToValueRequiresSingleField)
}

fn fake_contract_to_value_requires_record(
    _iteration: Iteration<'_>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error> {
    Err(iterate_contract::Error::ToValueRequiresRecord)
}

#[test]
fn iterate_resolver_success() {
    let operations = [IterationOperation::Take(1), IterationOperation::Iter];

    let iteration = Iteration {
        operations: &operations,
    };

    let result = iterate_resolver::resolve(fake_contract_success, iteration, receive);
    assert_eq!(result, Ok(()));
}

#[test]
fn iterate_resolver_translates_error() {
    let operations = [IterationOperation::Iter];

    let iteration = Iteration {
        operations: &operations,
    };

    let result = iterate_resolver::resolve(fake_contract_unavailable, iteration, receive);
    assert_eq!(result, Err(iterate::Error::IterationUnavailable));
}

#[test]
fn iterate_resolver_translates_to_value_requires_single_field() {
    let operations = [IterationOperation::ToValue];

    let iteration = Iteration {
        operations: &operations,
    };

    let result = iterate_resolver::resolve(
        fake_contract_to_value_requires_single_field,
        iteration,
        receive,
    );
    assert_eq!(result, Err(iterate::Error::ToValueRequiresSingleField));
}

#[test]
fn iterate_resolver_translates_to_value_requires_record() {
    let operations = [IterationOperation::ToValue];

    let iteration = Iteration {
        operations: &operations,
    };

    let result =
        iterate_resolver::resolve(fake_contract_to_value_requires_record, iteration, receive);
    assert_eq!(result, Err(iterate::Error::ToValueRequiresRecord));
}
