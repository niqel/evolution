use evo_shell::agents::iteration_dispatcher;
use evo_shell::definitions::contracts::iterate as iterate_contract;
use evo_shell::definitions::requesters::construction_requester;
use evo_shell::definitions::structs::borrowed::construction::Construction;
use evo_shell::definitions::structs::borrowed::iteration::Iteration;
use evo_shell::definitions::structs::borrowed::iteration_operation::IterationOperation;
use evo_shell::definitions::structs::borrowed::selection::Selection;
use evo_shell::definitions::structs::borrowed::value::Value;
use evo_shell::definitions::structs::owned::flow::Flow;
use evo_shell::definitions::use_cases::iterate as iterate_use_case;

fn receive(construction: Construction<'_>) -> Flow {
    assert_eq!(construction, Construction::Value(Value::Unsigned(42)));
    Flow::Continue
}

fn fake_iterate_success<'iteration>(
    iteration: Iteration<'iteration>,
    request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    assert_eq!(iteration.operations.len(), 2);
    assert_eq!(iteration.operations[0], IterationOperation::Take(1));
    assert_eq!(iteration.operations[1], IterationOperation::Iter);

    let flow = request(Construction::Value(Value::Unsigned(42)));
    assert_eq!(flow, Flow::Continue);

    Ok(())
}

fn fake_iterate_unavailable<'iteration>(
    _iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    Err(iterate_contract::Error::Unavailable)
}

fn fake_iterate_field_not_found<'iteration>(
    iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    match iteration.operations[0] {
        IterationOperation::Select(selections) => match selections[1] {
            Selection::Field(name) => Err(iterate_contract::Error::FieldNotFound(name)),
            _ => panic!("expected Selection::Field"),
        },
        _ => panic!("expected IterationOperation::Select"),
    }
}

#[test]
fn iteration_dispatcher_success() {
    let operations = [IterationOperation::Take(1), IterationOperation::Iter];

    let iteration = Iteration {
        operations: &operations,
    };

    let agent: iterate_use_case::Iterate = iteration_dispatcher::ITERATE;
    let result = agent(iteration, receive, fake_iterate_success);
    assert_eq!(result, Ok(()));
}

#[test]
fn iteration_dispatcher_translates_error() {
    let operations = [IterationOperation::Iter];

    let iteration = Iteration {
        operations: &operations,
    };

    let agent: iterate_use_case::Iterate = iteration_dispatcher::ITERATE;
    let result = agent(iteration, receive, fake_iterate_unavailable);
    assert_eq!(result, Err(iterate_use_case::Error::IterationUnavailable));
}

#[test]
fn iteration_dispatcher_translates_field_not_found() {
    let selections = [Selection::Field("name"), Selection::Field("missing")];
    let operations = [IterationOperation::Select(&selections)];

    let iteration = Iteration {
        operations: &operations,
    };

    let agent: iterate_use_case::Iterate = iteration_dispatcher::ITERATE;
    let result = agent(iteration, receive, fake_iterate_field_not_found);
    assert_eq!(
        result,
        Err(iterate_use_case::Error::FieldNotFound("missing"))
    );
}
