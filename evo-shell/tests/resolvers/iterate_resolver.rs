use evo_shell::definitions::contracts::iterate as iterate_contract;
use evo_shell::definitions::requesters::construction_requester;
use evo_shell::definitions::structs::borrowed::condition::Condition;
use evo_shell::definitions::structs::borrowed::condition_expression::ConditionExpression;
use evo_shell::definitions::structs::borrowed::construction::Construction;
use evo_shell::definitions::structs::borrowed::iteration::Iteration;
use evo_shell::definitions::structs::borrowed::iteration_operation::IterationOperation;
use evo_shell::definitions::structs::borrowed::selection::Selection;
use evo_shell::definitions::structs::borrowed::value::Value;
use evo_shell::definitions::structs::owned::condition_operator::ConditionOperator;
use evo_shell::definitions::structs::owned::flow::Flow;
use evo_shell::definitions::use_cases::iterate;
use evo_shell::resolvers::iterate_resolver;

fn receive(construction: Construction<'_>) -> Flow {
    assert_eq!(construction, Construction::Value(Value::Unsigned(42)));
    Flow::Continue
}

fn fake_contract_success<'iteration>(
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

fn fake_contract_unavailable<'iteration>(
    _iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    Err(iterate_contract::Error::Unavailable)
}

fn fake_contract_field_not_found<'iteration>(
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

fn fake_contract_comparison_type_mismatch<'iteration>(
    iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    match iteration.operations[0] {
        IterationOperation::Filter(ConditionExpression::Condition(condition)) => Err(
            iterate_contract::Error::ComparisonTypeMismatch(condition.field),
        ),
        _ => panic!("expected IterationOperation::Filter"),
    }
}

fn fake_contract_external_type_incompatible<'iteration>(
    iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    match iteration.operations[0] {
        IterationOperation::Select(selections) => match selections[0] {
            Selection::Field(name) => Err(iterate_contract::Error::ExternalTypeIncompatible(name)),
            _ => panic!("expected Selection::Field"),
        },
        _ => panic!("expected IterationOperation::Select"),
    }
}

fn fake_contract_provider_incompatible<'iteration>(
    _iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    Err(iterate_contract::Error::ProviderIncompatible)
}

fn fake_contract_to_value_requires_single_field<'iteration>(
    _iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    Err(iterate_contract::Error::ToValueRequiresSingleField)
}

fn fake_contract_to_value_requires_record<'iteration>(
    _iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
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
fn iterate_resolver_translates_field_not_found() {
    let selections = [Selection::Field("name"), Selection::Field("missing")];
    let operations = [IterationOperation::Select(&selections)];

    let iteration = Iteration {
        operations: &operations,
    };

    let result = iterate_resolver::resolve(fake_contract_field_not_found, iteration, receive);
    assert_eq!(result, Err(iterate::Error::FieldNotFound("missing")));
}

#[test]
fn iterate_resolver_translates_comparison_type_mismatch() {
    let condition = Condition {
        field: "size",
        operator: ConditionOperator::GreaterThan,
        value: Value::Text("hello"),
    };
    let operations = [IterationOperation::Filter(ConditionExpression::Condition(
        condition,
    ))];

    let iteration = Iteration {
        operations: &operations,
    };

    let result =
        iterate_resolver::resolve(fake_contract_comparison_type_mismatch, iteration, receive);
    assert_eq!(result, Err(iterate::Error::ComparisonTypeMismatch("size")));
}

#[test]
fn iterate_resolver_translates_external_type_incompatible() {
    let selections = [Selection::Field("created")];
    let operations = [IterationOperation::Select(&selections)];

    let iteration = Iteration {
        operations: &operations,
    };

    let result =
        iterate_resolver::resolve(fake_contract_external_type_incompatible, iteration, receive);
    assert_eq!(
        result,
        Err(iterate::Error::ExternalTypeIncompatible("created"))
    );
}

#[test]
fn iterate_resolver_translates_provider_incompatible() {
    let operations = [IterationOperation::Iter];

    let iteration = Iteration {
        operations: &operations,
    };

    let result = iterate_resolver::resolve(fake_contract_provider_incompatible, iteration, receive);
    assert_eq!(result, Err(iterate::Error::ProviderIncompatible));
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
