use evo_query::definitions::structs::borrowed::condition::Condition;
use evo_query::definitions::structs::borrowed::condition_expression::ConditionExpression;
use evo_query::definitions::structs::borrowed::iteration::Iteration;
use evo_query::definitions::structs::borrowed::iteration_operation::IterationOperation;
use evo_query::definitions::structs::borrowed::selection::Selection;
use evo_query::definitions::structs::owned::condition_operator::ConditionOperator;
use evo_values::definitions::value::Value;

#[test]
fn iteration_ordered_pipeline() {
    let selections = [Selection::Field("name"), Selection::Field("size")];
    let operations = [
        IterationOperation::Select(&selections),
        IterationOperation::Skip(20),
        IterationOperation::Take(10),
    ];

    let iteration = Iteration {
        operations: &operations,
    };

    assert_eq!(iteration.operations.len(), 3);

    match iteration.operations[0] {
        IterationOperation::Select(selected_fields) => {
            assert_eq!(selected_fields.len(), 2);
            assert_eq!(selected_fields[0], Selection::Field("name"));
            assert_eq!(selected_fields[1], Selection::Field("size"));
        }
        _ => panic!("expected IterationOperation::Select"),
    }

    assert_eq!(iteration.operations[1], IterationOperation::Skip(20));
    assert_eq!(iteration.operations[2], IterationOperation::Take(10));
}

#[test]
fn iteration_skip_then_take_order() {
    let operations = [IterationOperation::Skip(5), IterationOperation::Take(10)];

    let iteration = Iteration {
        operations: &operations,
    };

    assert_eq!(iteration.operations.len(), 2);
    assert_eq!(iteration.operations[0], IterationOperation::Skip(5));
    assert_eq!(iteration.operations[1], IterationOperation::Take(10));
}

#[test]
fn iteration_take_then_skip_order() {
    let operations = [IterationOperation::Take(10), IterationOperation::Skip(5)];

    let iteration = Iteration {
        operations: &operations,
    };

    assert_eq!(iteration.operations.len(), 2);
    assert_eq!(iteration.operations[0], IterationOperation::Take(10));
    assert_eq!(iteration.operations[1], IterationOperation::Skip(5));
}

#[test]
fn iteration_filter_skip_take_order() {
    let condition = Condition {
        field: "active",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    };
    let filter_expr = ConditionExpression::Condition(condition);

    let operations = [
        IterationOperation::Filter(filter_expr),
        IterationOperation::Skip(10),
        IterationOperation::Take(5),
    ];

    let iteration = Iteration {
        operations: &operations,
    };

    assert_eq!(iteration.operations.len(), 3);
    assert_eq!(
        iteration.operations[0],
        IterationOperation::Filter(filter_expr)
    );
    assert_eq!(iteration.operations[1], IterationOperation::Skip(10));
    assert_eq!(iteration.operations[2], IterationOperation::Take(5));
}

#[test]
fn iteration_empty_pipeline() {
    let operations: [IterationOperation<'_>; 0] = [];

    let iteration = Iteration {
        operations: &operations,
    };

    assert!(iteration.operations.is_empty());
}

#[test]
fn iteration_single_operation() {
    let operations = [IterationOperation::Count];

    let iteration = Iteration {
        operations: &operations,
    };

    assert_eq!(iteration.operations.len(), 1);
    assert_eq!(iteration.operations[0], IterationOperation::Count);
}

#[test]
fn iteration_copy() {
    let operations = [IterationOperation::Take(5), IterationOperation::Count];

    let original = Iteration {
        operations: &operations,
    };

    let copied = original;

    assert_eq!(original, copied);
}
