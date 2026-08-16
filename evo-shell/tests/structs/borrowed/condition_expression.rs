use evo_shell::definitions::structs::borrowed::condition::Condition;
use evo_shell::definitions::structs::borrowed::condition_expression::ConditionExpression;
use evo_shell::definitions::structs::borrowed::value::Value;
use evo_shell::definitions::structs::owned::condition_operator::ConditionOperator;

#[test]
fn condition_expression_condition() {
    let condition = Condition {
        field: "name",
        operator: ConditionOperator::Equal,
        value: Value::Text("config.evo"),
    };

    let expression = ConditionExpression::Condition(condition);

    assert_eq!(expression, ConditionExpression::Condition(condition));

    match expression {
        ConditionExpression::Condition(inner) => {
            assert_eq!(inner.field, "name");
            assert_eq!(inner.operator, ConditionOperator::Equal);
            assert_eq!(inner.value, Value::Text("config.evo"));
        }
        _ => panic!("expected ConditionExpression::Condition"),
    }
}

#[test]
fn condition_expression_and() {
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

    let children = [a, b];
    let expression = ConditionExpression::And(&children);

    match expression {
        ConditionExpression::And(items) => {
            assert_eq!(items.len(), 2);
            assert_eq!(items[0], a);
            assert_eq!(items[1], b);
        }
        _ => panic!("expected ConditionExpression::And"),
    }
}

#[test]
fn condition_expression_or() {
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

    let children = [a, b];
    let expression = ConditionExpression::Or(&children);

    match expression {
        ConditionExpression::Or(items) => {
            assert_eq!(items.len(), 2);
            assert_eq!(items[0], a);
            assert_eq!(items[1], b);
        }
        _ => panic!("expected ConditionExpression::Or"),
    }
}

#[test]
fn condition_expression_and_or_grouping() {
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

    match expression {
        ConditionExpression::Or(items) => {
            assert_eq!(items.len(), 2);
            match items[0] {
                ConditionExpression::And(and_items) => {
                    assert_eq!(and_items.len(), 2);
                    assert_eq!(and_items[0], a);
                    assert_eq!(and_items[1], b);
                }
                _ => panic!("expected ConditionExpression::And as first child"),
            }
            assert_eq!(items[1], c);
        }
        _ => panic!("expected ConditionExpression::Or as root"),
    }
}

#[test]
fn condition_expression_or_and_grouping() {
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

    let or_children = [b, c];
    let or_expression = ConditionExpression::Or(&or_children);

    let and_children = [a, or_expression];
    let expression = ConditionExpression::And(&and_children);

    match expression {
        ConditionExpression::And(items) => {
            assert_eq!(items.len(), 2);
            assert_eq!(items[0], a);
            match items[1] {
                ConditionExpression::Or(or_items) => {
                    assert_eq!(or_items.len(), 2);
                    assert_eq!(or_items[0], b);
                    assert_eq!(or_items[1], c);
                }
                _ => panic!("expected ConditionExpression::Or as second child"),
            }
        }
        _ => panic!("expected ConditionExpression::And as root"),
    }
}

#[test]
fn condition_expression_grouping_inequality() {
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
    let or_children_for_first = [and_expression, c];
    let expression_one = ConditionExpression::Or(&or_children_for_first);

    let or_children_for_second = [b, c];
    let or_expression = ConditionExpression::Or(&or_children_for_second);
    let and_children_for_second = [a, or_expression];
    let expression_two = ConditionExpression::And(&and_children_for_second);

    assert_ne!(expression_one, expression_two);
}

#[test]
fn condition_expression_copy() {
    let condition = Condition {
        field: "active",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    };

    let original = ConditionExpression::Condition(condition);
    let copied = original;

    assert_eq!(original, copied);
}
