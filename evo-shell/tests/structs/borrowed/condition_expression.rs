use evo_shell::definitions::structs::borrowed::between_condition::BetweenCondition;
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
fn condition_expression_between() {
    let between = BetweenCondition {
        field: "age",
        lower: Value::Unsigned(18),
        upper: Value::Unsigned(65),
    };

    let expression = ConditionExpression::Between(between);

    assert_eq!(expression, ConditionExpression::Between(between));

    match expression {
        ConditionExpression::Between(inner) => {
            assert_eq!(inner.field, "age");
            assert_eq!(inner.lower, Value::Unsigned(18));
            assert_eq!(inner.upper, Value::Unsigned(65));
        }
        _ => panic!("expected ConditionExpression::Between"),
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
fn condition_expression_between_and_condition() {
    let between_expression = ConditionExpression::Between(BetweenCondition {
        field: "age",
        lower: Value::Unsigned(18),
        upper: Value::Unsigned(65),
    });

    let condition_expression = ConditionExpression::Condition(Condition {
        field: "active",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    });

    let children = [between_expression, condition_expression];
    let expression = ConditionExpression::And(&children);

    match expression {
        ConditionExpression::And(items) => {
            assert_eq!(items.len(), 2);
            assert_eq!(items[0], between_expression);
            assert_eq!(items[1], condition_expression);
            match items[0] {
                ConditionExpression::Between(_) => {}
                _ => panic!("expected ConditionExpression::Between as first child"),
            }
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
fn condition_expression_between_or_condition() {
    let between_expression = ConditionExpression::Between(BetweenCondition {
        field: "score",
        lower: Value::Unsigned(50),
        upper: Value::Unsigned(100),
    });

    let condition_expression = ConditionExpression::Condition(Condition {
        field: "active",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    });

    let children = [between_expression, condition_expression];
    let expression = ConditionExpression::Or(&children);

    match expression {
        ConditionExpression::Or(items) => {
            assert_eq!(items.len(), 2);
            assert_eq!(items[0], between_expression);
            assert_eq!(items[1], condition_expression);
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
fn condition_expression_between_grouping() {
    let between = ConditionExpression::Between(BetweenCondition {
        field: "age",
        lower: Value::Unsigned(18),
        upper: Value::Unsigned(65),
    });

    let active = ConditionExpression::Condition(Condition {
        field: "active",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    });

    let score = ConditionExpression::Condition(Condition {
        field: "score",
        operator: ConditionOperator::GreaterThan,
        value: Value::Unsigned(100),
    });

    let and_children = [between, active];
    let and_expression = ConditionExpression::And(&and_children);

    let or_children = [and_expression, score];
    let expression = ConditionExpression::Or(&or_children);

    match expression {
        ConditionExpression::Or(items) => {
            assert_eq!(items.len(), 2);
            match items[0] {
                ConditionExpression::And(and_items) => {
                    assert_eq!(and_items.len(), 2);
                    assert_eq!(and_items[0], between);
                    assert_eq!(and_items[1], active);
                }
                _ => panic!("expected ConditionExpression::And as first child"),
            }
            assert_eq!(items[1], score);
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
fn condition_expression_between_grouping_inequality() {
    let between = ConditionExpression::Between(BetweenCondition {
        field: "age",
        lower: Value::Unsigned(18),
        upper: Value::Unsigned(65),
    });

    let active = ConditionExpression::Condition(Condition {
        field: "active",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    });

    let score = ConditionExpression::Condition(Condition {
        field: "score",
        operator: ConditionOperator::GreaterThan,
        value: Value::Unsigned(100),
    });

    let and_children = [between, active];
    let and_expression = ConditionExpression::And(&and_children);
    let or_children_for_first = [and_expression, score];
    let expression_one = ConditionExpression::Or(&or_children_for_first);

    let or_children_for_second = [active, score];
    let or_expression = ConditionExpression::Or(&or_children_for_second);
    let and_children_for_second = [between, or_expression];
    let expression_two = ConditionExpression::And(&and_children_for_second);

    assert_ne!(expression_one, expression_two);
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

#[test]
fn condition_expression_between_copy() {
    let between = BetweenCondition {
        field: "age",
        lower: Value::Unsigned(18),
        upper: Value::Unsigned(65),
    };

    let original = ConditionExpression::Between(between);
    let copied = original;

    assert_eq!(original, copied);
}
