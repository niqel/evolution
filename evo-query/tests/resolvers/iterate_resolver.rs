use evo_query::definitions::contracts::iterate as iterate_contract;
use evo_query::definitions::requesters::construction_requester;
use evo_query::definitions::structs::borrowed::condition::Condition;
use evo_query::definitions::structs::borrowed::condition_expression::ConditionExpression;
use evo_query::definitions::structs::borrowed::construction::Construction;
use evo_query::definitions::structs::borrowed::iteration::Iteration;
use evo_query::definitions::structs::borrowed::iteration_operation::IterationOperation;
use evo_query::definitions::structs::borrowed::selection::Selection;
use evo_query::definitions::structs::owned::condition_operator::ConditionOperator;
use evo_query::definitions::structs::owned::flow::Flow;
use evo_query::definitions::use_cases::iterate;
use evo_query::resolvers::iterate_resolver;
use evo_values::definitions::value::Value;

fn receive(construction: Construction<'_>) -> Flow {
    assert_eq!(construction, Construction::Value(Value::Uint64(42)));
    Flow::Continue
}

fn fake_contract_success<'iteration>(
    iteration: Iteration<'iteration>,
    request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    assert_eq!(iteration.operations.len(), 1);
    assert_eq!(iteration.operations[0], IterationOperation::Take(1));

    let flow = request(Construction::Value(Value::Uint64(42)));
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
    match &iteration.operations[0] {
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
    match &iteration.operations[0] {
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
    match &iteration.operations[0] {
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

fn fake_contract_text_expected<'iteration>(
    _iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    Err(iterate_contract::Error::TextExpected)
}

fn fake_contract_unsigned_expected<'iteration>(
    _iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    Err(iterate_contract::Error::UnsignedExpected)
}

fn fake_contract_substring_out_of_bounds<'iteration>(
    _iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    Err(iterate_contract::Error::SubstringOutOfBounds)
}

fn fake_contract_replace_empty_pattern<'iteration>(
    _iteration: Iteration<'iteration>,
    _request: construction_requester::Request,
) -> Result<(), iterate_contract::Error<'iteration>> {
    Err(iterate_contract::Error::ReplaceEmptyPattern)
}

#[test]
fn iterate_resolver_success() {
    let operations = [IterationOperation::Take(1)];

    let iteration = Iteration {
        operations: &operations,
    };

    let result = iterate_resolver::resolve(fake_contract_success, iteration, receive);
    assert_eq!(result, Ok(()));
}

#[test]
fn iterate_resolver_translates_error() {
    let operations: [IterationOperation<'_>; 0] = [];

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
        value: Value::String("hello"),
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
    let operations: [IterationOperation<'_>; 0] = [];

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

#[test]
fn iterate_resolver_translates_text_expected() {
    let operations: [IterationOperation<'_>; 0] = [];
    let iteration = Iteration {
        operations: &operations,
    };
    let result = iterate_resolver::resolve(fake_contract_text_expected, iteration, receive);
    assert_eq!(result, Err(iterate::Error::TextExpected));
}

#[test]
fn iterate_resolver_translates_unsigned_expected() {
    let operations: [IterationOperation<'_>; 0] = [];
    let iteration = Iteration {
        operations: &operations,
    };
    let result = iterate_resolver::resolve(fake_contract_unsigned_expected, iteration, receive);
    assert_eq!(result, Err(iterate::Error::UnsignedExpected));
}

#[test]
fn iterate_resolver_translates_substring_out_of_bounds() {
    let operations: [IterationOperation<'_>; 0] = [];
    let iteration = Iteration {
        operations: &operations,
    };
    let result =
        iterate_resolver::resolve(fake_contract_substring_out_of_bounds, iteration, receive);
    assert_eq!(result, Err(iterate::Error::SubstringOutOfBounds));
}

#[test]
fn iterate_resolver_translates_replace_empty_pattern() {
    let operations: [IterationOperation<'_>; 0] = [];
    let iteration = Iteration {
        operations: &operations,
    };
    let result = iterate_resolver::resolve(fake_contract_replace_empty_pattern, iteration, receive);
    assert_eq!(result, Err(iterate::Error::ReplaceEmptyPattern));
}
