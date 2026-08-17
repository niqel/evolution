use evo_shell::definitions::structs::borrowed::condition::Condition;
use evo_shell::definitions::structs::borrowed::condition_expression::ConditionExpression;
use evo_shell::definitions::structs::borrowed::iteration_operation::IterationOperation;
use evo_shell::definitions::structs::borrowed::selection::Selection;
use evo_shell::definitions::structs::owned::condition_operator::ConditionOperator;
use evo_values::definitions::value::Value;

#[test]
fn iteration_operation_filter() {
    let condition = Condition {
        field: "name",
        operator: ConditionOperator::Equal,
        value: Value::Text("config.evo"),
    };

    let expression = ConditionExpression::Condition(condition);
    let operation = IterationOperation::Filter(expression);

    assert_eq!(operation, IterationOperation::Filter(expression));
    assert_ne!(operation, IterationOperation::ToValue);

    match operation {
        IterationOperation::Filter(filter_expression) => {
            assert_eq!(filter_expression, expression);
        }
        _ => panic!("expected IterationOperation::Filter"),
    }
}

#[test]
fn iteration_operation_filter_and_expression() {
    let a = ConditionExpression::Condition(Condition {
        field: "name",
        operator: ConditionOperator::Equal,
        value: Value::Text("config.evo"),
    });

    let b = ConditionExpression::Condition(Condition {
        field: "size",
        operator: ConditionOperator::GreaterThan,
        value: Value::Unsigned(1024),
    });

    let expressions = [a, b];
    let expression = ConditionExpression::And(&expressions);
    let operation = IterationOperation::Filter(expression);

    match operation {
        IterationOperation::Filter(ConditionExpression::And(children)) => {
            assert_eq!(children.len(), 2);
            assert_eq!(children[0], a);
            assert_eq!(children[1], b);
        }
        _ => panic!("expected IterationOperation::Filter with And"),
    }
}

#[test]
fn iteration_operation_filter_grouped_expression() {
    let a = ConditionExpression::Condition(Condition {
        field: "name",
        operator: ConditionOperator::Equal,
        value: Value::Text("config.evo"),
    });

    let b = ConditionExpression::Condition(Condition {
        field: "size",
        operator: ConditionOperator::GreaterThan,
        value: Value::Unsigned(1024),
    });

    let c = ConditionExpression::Condition(Condition {
        field: "active",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    });

    let and_children = [a, b];
    let and_expression = ConditionExpression::And(&and_children);
    let or_children = [and_expression, c];
    let expression = ConditionExpression::Or(&or_children);
    let operation = IterationOperation::Filter(expression);

    match operation {
        IterationOperation::Filter(ConditionExpression::Or(children)) => {
            assert_eq!(children.len(), 2);
            match children[0] {
                ConditionExpression::And(and_items) => {
                    assert_eq!(and_items.len(), 2);
                    assert_eq!(and_items[0], a);
                    assert_eq!(and_items[1], b);
                }
                _ => panic!("expected And as first child"),
            }
            assert_eq!(children[1], c);
        }
        _ => panic!("expected IterationOperation::Filter with Or"),
    }
}

#[test]
fn iteration_operation_select() {
    let selections = [Selection::Field("name"), Selection::Field("size")];
    let operation = IterationOperation::Select(&selections);

    match operation {
        IterationOperation::Select(selected_fields) => {
            assert_eq!(selected_fields.len(), 2);
            assert_eq!(selected_fields[0], Selection::Field("name"));
            assert_eq!(selected_fields[1], Selection::Field("size"));
        }
        _ => panic!("expected IterationOperation::Select"),
    }
}

#[test]
fn iteration_operation_to_value() {
    let operation = IterationOperation::ToValue;

    assert_eq!(operation, IterationOperation::ToValue);
}

#[test]
fn iteration_operation_take() {
    let operation = IterationOperation::Take(10);
    assert_eq!(operation, IterationOperation::Take(10));
    assert_ne!(operation, IterationOperation::Take(5));

    match operation {
        IterationOperation::Take(count) => assert_eq!(count, 10),
        _ => panic!("expected IterationOperation::Take"),
    }
}

#[test]
fn iteration_operation_take_zero() {
    let operation = IterationOperation::Take(0);
    assert_eq!(operation, IterationOperation::Take(0));
    assert_ne!(operation, IterationOperation::Take(1));

    match operation {
        IterationOperation::Take(count) => assert_eq!(count, 0),
        _ => panic!("expected IterationOperation::Take"),
    }
}

#[test]
fn iteration_operation_skip() {
    let operation = IterationOperation::Skip(20);
    assert_eq!(operation, IterationOperation::Skip(20));
    assert_ne!(operation, IterationOperation::Skip(10));

    match operation {
        IterationOperation::Skip(count) => assert_eq!(count, 20),
        _ => panic!("expected IterationOperation::Skip"),
    }
}

#[test]
fn iteration_operation_skip_zero() {
    let operation = IterationOperation::Skip(0);
    assert_eq!(operation, IterationOperation::Skip(0));
    assert_ne!(operation, IterationOperation::Skip(1));

    match operation {
        IterationOperation::Skip(count) => assert_eq!(count, 0),
        _ => panic!("expected IterationOperation::Skip"),
    }
}

#[test]
fn iteration_operation_unit_variants() {
    assert_eq!(IterationOperation::ToValue, IterationOperation::ToValue);
    assert_eq!(IterationOperation::First, IterationOperation::First);
    assert_eq!(IterationOperation::Last, IterationOperation::Last);
    assert_eq!(IterationOperation::Count, IterationOperation::Count);

    assert_ne!(IterationOperation::ToValue, IterationOperation::Count);
    assert_ne!(IterationOperation::First, IterationOperation::Last);
    assert_ne!(IterationOperation::Count, IterationOperation::First);
}

#[test]
fn iteration_operation_copy() {
    let selections = [Selection::Field("name")];
    let original = IterationOperation::Select(&selections);
    let copied = original;

    assert_eq!(original, copied);

    let original_to_value = IterationOperation::ToValue;
    let copied_to_value = original_to_value;

    assert_eq!(original_to_value, copied_to_value);

    let condition = Condition {
        field: "name",
        operator: ConditionOperator::Equal,
        value: Value::Text("config.evo"),
    };
    let expression = ConditionExpression::Condition(condition);
    let original_filter = IterationOperation::Filter(expression);
    let copied_filter = original_filter;

    assert_eq!(original_filter, copied_filter);

    let original_take = IterationOperation::Take(15);
    let copied_take = original_take;

    assert_eq!(original_take, copied_take);

    let original_skip = IterationOperation::Skip(25);
    let copied_skip = original_skip;

    assert_eq!(original_skip, copied_skip);
}
