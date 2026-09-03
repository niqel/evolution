use evo_query::definitions::contracts::iterate;
use evo_query::definitions::requesters::construction_requester;
use evo_query::definitions::structs::borrowed::condition::Condition;
use evo_query::definitions::structs::borrowed::condition_expression::ConditionExpression;
use evo_query::definitions::structs::borrowed::construction::Construction;
use evo_query::definitions::structs::borrowed::iteration::Iteration;
use evo_query::definitions::structs::borrowed::iteration_operation::IterationOperation;
use evo_query::definitions::structs::borrowed::selection::Selection;
use evo_query::definitions::structs::owned::condition_operator::ConditionOperator;
use evo_query::definitions::structs::owned::flow::Flow;
use evo_values::definitions::value::Value;

fn receive_construction_continue(construction: Construction<'_>) -> Flow {
    assert_eq!(construction, Construction::Value(Value::Uint64(1)));
    Flow::Continue
}

fn receive_construction_stop(construction: Construction<'_>) -> Flow {
    assert_eq!(construction, Construction::Value(Value::Uint64(1)));
    Flow::Stop
}

fn fake_iterate_success<'iteration>(
    iteration: Iteration<'iteration>,
    request: construction_requester::Request,
) -> Result<(), iterate::Error<'iteration>> {
    assert_eq!(iteration.operations.len(), 2);
    assert_eq!(iteration.operations[1], IterationOperation::Take(1));

    let flow = request(Construction::Value(Value::Uint64(1)));
    assert_eq!(flow, Flow::Continue);

    Ok(())
}

fn fake_iterate_handles_stop<'iteration>(
    iteration: Iteration<'iteration>,
    request: construction_requester::Request,
) -> Result<(), iterate::Error<'iteration>> {
    assert_eq!(iteration.operations.len(), 1);
    assert_eq!(iteration.operations[0], IterationOperation::Take(1));

    let flow = request(Construction::Value(Value::Uint64(1)));
    assert_eq!(flow, Flow::Stop);

    Ok(())
}

fn fake_iterate_zero_results<'iteration>(
    _iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate::Error<'iteration>> {
    Ok(())
}

fn fake_iterate_multiple_results<'iteration>(
    _iteration: Iteration<'iteration>,
    request: construction_requester::Request,
) -> Result<(), iterate::Error<'iteration>> {
    let flow_1 = request(Construction::Value(Value::Uint64(1)));
    if flow_1 == Flow::Stop {
        return Ok(());
    }
    let flow_2 = request(Construction::Value(Value::Uint64(2)));
    if flow_2 == Flow::Stop {
        return Ok(());
    }
    let _flow_3 = request(Construction::Value(Value::Uint64(3)));
    Ok(())
}

fn fake_iterate_stop_prevents_subsequent_results<'iteration>(
    _iteration: Iteration<'iteration>,
    request: construction_requester::Request,
) -> Result<(), iterate::Error<'iteration>> {
    let flow_1 = request(Construction::Value(Value::Uint64(1)));
    if flow_1 == Flow::Stop {
        return Ok(());
    }
    let flow_2 = request(Construction::Value(Value::Uint64(2)));
    if flow_2 == Flow::Stop {
        return Ok(());
    }
    panic!("third result should not be requested after Flow::Stop");
}

fn unavailable_iterate<'iteration>(
    _iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate::Error<'iteration>> {
    Err(iterate::Error::Unavailable)
}

fn to_value_requires_single_field<'iteration>(
    _iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate::Error<'iteration>> {
    Err(iterate::Error::ToValueRequiresSingleField)
}

fn to_value_requires_record<'iteration>(
    _iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate::Error<'iteration>> {
    Err(iterate::Error::ToValueRequiresRecord)
}

fn field_not_found<'iteration>(
    iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate::Error<'iteration>> {
    match &iteration.operations[0] {
        IterationOperation::Select(selections) => match selections[1] {
            Selection::Field(name) => Err(iterate::Error::FieldNotFound(name)),
            _ => panic!("expected Selection::Field"),
        },
        _ => panic!("expected IterationOperation::Select"),
    }
}

fn comparison_type_mismatch<'iteration>(
    iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate::Error<'iteration>> {
    match &iteration.operations[0] {
        IterationOperation::Filter(ConditionExpression::Condition(condition)) => {
            Err(iterate::Error::ComparisonTypeMismatch(condition.field))
        }
        _ => panic!("expected IterationOperation::Filter with Condition"),
    }
}

fn external_type_incompatible<'iteration>(
    iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate::Error<'iteration>> {
    match &iteration.operations[0] {
        IterationOperation::Select(selections) => match selections[0] {
            Selection::Field(name) => Err(iterate::Error::ExternalTypeIncompatible(name)),
            _ => panic!("expected Selection::Field"),
        },
        _ => panic!("expected IterationOperation::Select"),
    }
}

fn provider_incompatible<'iteration>(
    _iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate::Error<'iteration>> {
    Err(iterate::Error::ProviderIncompatible)
}

#[test]
fn iterate_contract_signature_and_success() {
    let selections = [Selection::Field("name")];
    let operations = [
        IterationOperation::Select(&selections),
        IterationOperation::Take(1),
    ];

    let iteration = Iteration {
        operations: &operations,
    };

    let contract: iterate::Iterate = fake_iterate_success;
    assert_eq!(contract(iteration, receive_construction_continue), Ok(()));
}

#[test]
fn iterate_contract_handles_flow_stop() {
    let operations = [IterationOperation::Take(1)];

    let iteration = Iteration {
        operations: &operations,
    };

    let contract: iterate::Iterate = fake_iterate_handles_stop;
    assert_eq!(contract(iteration, receive_construction_stop), Ok(()));
}

#[test]
fn iterate_contract_zero_results() {
    let operations: [IterationOperation<'_>; 0] = [];

    let iteration = Iteration {
        operations: &operations,
    };

    let contract: iterate::Iterate = fake_iterate_zero_results;
    assert_eq!(
        contract(iteration, |_| panic!(
            "requester should not be called for zero results"
        )),
        Ok(())
    );
}

#[test]
fn iterate_contract_multiple_results_continue() {
    let operations = [IterationOperation::Take(3)];

    let iteration = Iteration {
        operations: &operations,
    };

    let contract: iterate::Iterate = fake_iterate_multiple_results;
    assert_eq!(contract(iteration, |_| Flow::Continue), Ok(()));
}

#[test]
fn iterate_contract_stop_prevents_next_result() {
    let operations = [IterationOperation::Take(3)];

    let iteration = Iteration {
        operations: &operations,
    };

    let contract: iterate::Iterate = fake_iterate_stop_prevents_subsequent_results;
    let result = contract(iteration, |construction| match construction {
        Construction::Value(Value::Uint64(1)) => Flow::Continue,
        Construction::Value(Value::Uint64(2)) => Flow::Stop,
        _ => panic!("unexpected construction"),
    });

    assert_eq!(result, Ok(()));
}

#[test]
fn iterate_contract_error() {
    let operations: [IterationOperation<'_>; 0] = [];

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

#[test]
fn iterate_contract_field_not_found_error() {
    let selections = [Selection::Field("name"), Selection::Field("missing")];
    let operations = [IterationOperation::Select(&selections)];

    let iteration = Iteration {
        operations: &operations,
    };

    let contract: iterate::Iterate = field_not_found;
    assert_eq!(
        contract(iteration, receive_construction_continue),
        Err(iterate::Error::FieldNotFound("missing"))
    );
}

#[test]
fn iterate_contract_comparison_type_mismatch_error() {
    let condition = Condition {
        field: "size",
        operator: ConditionOperator::GreaterThan,
        value: Value::String("hello"),
    };
    let operations = [IterationOperation::Filter(ConditionExpression::Condition(
        condition,
    ))];

    let iteration = Iteration {
        operations: &operations,
    };

    let contract: iterate::Iterate = comparison_type_mismatch;
    assert_eq!(
        contract(iteration, receive_construction_continue),
        Err(iterate::Error::ComparisonTypeMismatch("size"))
    );
}

#[test]
fn iterate_contract_external_type_incompatible_error() {
    let selections = [Selection::Field("created")];
    let operations = [IterationOperation::Select(&selections)];

    let iteration = Iteration {
        operations: &operations,
    };

    let contract: iterate::Iterate = external_type_incompatible;
    assert_eq!(
        contract(iteration, receive_construction_continue),
        Err(iterate::Error::ExternalTypeIncompatible("created"))
    );
}

#[test]
fn iterate_contract_provider_incompatible_error() {
    let operations: [IterationOperation<'_>; 0] = [];

    let iteration = Iteration {
        operations: &operations,
    };

    let contract: iterate::Iterate = provider_incompatible;
    assert_eq!(
        contract(iteration, receive_construction_continue),
        Err(iterate::Error::ProviderIncompatible)
    );
}
