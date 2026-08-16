use evo_shell::definitions::contracts::iterate;
use evo_shell::definitions::requesters::construction_requester;
use evo_shell::definitions::structs::borrowed::construction::Construction;
use evo_shell::definitions::structs::borrowed::iteration::Iteration;
use evo_shell::definitions::structs::borrowed::iteration_operation::IterationOperation;
use evo_shell::definitions::structs::borrowed::value::Value;
use evo_shell::definitions::structs::owned::flow::Flow;

fn receive_construction_continue(construction: Construction<'_>) -> Flow {
    assert_eq!(construction, Construction::Value(Value::Unsigned(1)));
    Flow::Continue
}

fn receive_construction_stop(construction: Construction<'_>) -> Flow {
    assert_eq!(construction, Construction::Value(Value::Unsigned(1)));
    Flow::Stop
}

fn fake_iterate_success(
    iteration: Iteration<'_>,
    request: construction_requester::Request,
) -> Result<(), iterate::Error> {
    assert_eq!(iteration.operations.len(), 3);
    assert_eq!(iteration.operations[1], IterationOperation::Take(1));
    assert_eq!(iteration.operations[2], IterationOperation::Iter);

    let flow = request(Construction::Value(Value::Unsigned(1)));
    assert_eq!(flow, Flow::Continue);

    Ok(())
}

fn fake_iterate_handles_stop(
    iteration: Iteration<'_>,
    request: construction_requester::Request,
) -> Result<(), iterate::Error> {
    assert_eq!(iteration.operations.len(), 1);
    assert_eq!(iteration.operations[0], IterationOperation::Iter);

    let flow = request(Construction::Value(Value::Unsigned(1)));
    assert_eq!(flow, Flow::Stop);

    Ok(())
}

fn unavailable_iterate(
    _iteration: Iteration<'_>,
    _request: construction_requester::Request,
) -> Result<(), iterate::Error> {
    Err(iterate::Error::Unavailable)
}

fn to_value_requires_single_field(
    _iteration: Iteration<'_>,
    _request: construction_requester::Request,
) -> Result<(), iterate::Error> {
    Err(iterate::Error::ToValueRequiresSingleField)
}

fn to_value_requires_record(
    _iteration: Iteration<'_>,
    _request: construction_requester::Request,
) -> Result<(), iterate::Error> {
    Err(iterate::Error::ToValueRequiresRecord)
}

#[test]
fn iterate_contract_signature_and_success() {
    let fields = ["name"];
    let operations = [
        IterationOperation::Select(&fields),
        IterationOperation::Take(1),
        IterationOperation::Iter,
    ];

    let iteration = Iteration {
        operations: &operations,
    };

    let contract: iterate::Iterate = fake_iterate_success;
    assert_eq!(contract(iteration, receive_construction_continue), Ok(()));
}

#[test]
fn iterate_contract_handles_flow_stop() {
    let operations = [IterationOperation::Iter];

    let iteration = Iteration {
        operations: &operations,
    };

    let contract: iterate::Iterate = fake_iterate_handles_stop;
    assert_eq!(contract(iteration, receive_construction_stop), Ok(()));
}

#[test]
fn iterate_contract_error() {
    let operations = [IterationOperation::Iter];

    let iteration = Iteration {
        operations: &operations,
    };

    let contract: iterate::Iterate = unavailable_iterate;
    assert_eq!(
        contract(iteration, receive_construction_continue),
        Err(iterate::Error::Unavailable)
    );
}

#[test]
fn iterate_contract_to_value_requires_single_field_error() {
    let operations = [IterationOperation::ToValue];

    let iteration = Iteration {
        operations: &operations,
    };

    let contract: iterate::Iterate = to_value_requires_single_field;
    assert_eq!(
        contract(iteration, receive_construction_continue),
        Err(iterate::Error::ToValueRequiresSingleField)
    );
}

#[test]
fn iterate_contract_to_value_requires_record_error() {
    let operations = [IterationOperation::ToValue];

    let iteration = Iteration {
        operations: &operations,
    };

    let contract: iterate::Iterate = to_value_requires_record;
    assert_eq!(
        contract(iteration, receive_construction_continue),
        Err(iterate::Error::ToValueRequiresRecord)
    );
}
