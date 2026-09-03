use evo_query::definitions::structs::borrowed::between_condition::BetweenCondition;
use evo_query::definitions::structs::borrowed::condition::Condition;
use evo_query::definitions::structs::borrowed::condition_expression::ConditionExpression;
use evo_query::definitions::structs::borrowed::in_condition::InCondition;
use evo_query::definitions::structs::owned::condition_operator::ConditionOperator;
use evo_values::definitions::value::Value;

#[test]
fn condition_expression_condition() {
    let condition = Condition {
        field: "name",
        operator: ConditionOperator::Equal,
        value: Value::String("config.evo"),
    };

    let expression = ConditionExpression::Condition(condition.clone());

    assert_eq!(expression, ConditionExpression::Condition(condition));

    match expression {
        ConditionExpression::Condition(inner) => {
            assert_eq!(inner.field, "name");
            assert_eq!(inner.operator, ConditionOperator::Equal);
            assert_eq!(inner.value, Value::String("config.evo"));
        }
        _ => panic!("expected ConditionExpression::Condition"),
    }
}

#[test]
fn condition_expression_between() {
    let between = BetweenCondition {
        field: "age",
        lower: Value::Uint64(18),
        upper: Value::Uint64(65),
    };

    let expression = ConditionExpression::Between(between.clone());

    assert_eq!(expression, ConditionExpression::Between(between));

    match expression {
        ConditionExpression::Between(inner) => {
            assert_eq!(inner.field, "age");
            assert_eq!(inner.lower, Value::Uint64(18));
            assert_eq!(inner.upper, Value::Uint64(65));
        }
        _ => panic!("expected ConditionExpression::Between"),
    }
}

#[test]
fn condition_expression_in() {
    let values = [Value::String("active"), Value::String("pending")];
    let in_condition = InCondition {
        field: "status",
        values: &values,
    };

    let expression = ConditionExpression::In(in_condition);

    assert_eq!(expression, ConditionExpression::In(in_condition));

    match expression {
        ConditionExpression::In(inner) => {
            assert_eq!(inner.field, "status");
            assert_eq!(inner.values.len(), 2);
            assert_eq!(inner.values[0], Value::String("active"));
            assert_eq!(inner.values[1], Value::String("pending"));
        }
        _ => panic!("expected ConditionExpression::In"),
    }
}

#[test]
fn condition_expression_not_in() {
    let values = [Value::String(".tmp"), Value::String(".bak")];
    let in_expression = ConditionExpression::In(InCondition {
        field: "extension",
        values: &values,
    });

    let expression = ConditionExpression::Not(&in_expression);

    match expression {
        ConditionExpression::Not(inner) => match *inner {
            ConditionExpression::In(in_cond) => {
                assert_eq!(in_cond.field, "extension");
                assert_eq!(in_cond.values.len(), 2);
                assert_eq!(in_cond.values[0], Value::String(".tmp"));
                assert_eq!(in_cond.values[1], Value::String(".bak"));
            }
            _ => panic!("expected ConditionExpression::In inside Not"),
        },
        _ => panic!("expected ConditionExpression::Not"),
    }
}

#[test]
fn condition_expression_in_and_not_in_are_structurally_different() {
    let values = [Value::String("active"), Value::String("pending")];
    let condition = InCondition {
        field: "status",
        values: &values,
    };

    let in_expression = ConditionExpression::In(condition);
    let not_in_expression = ConditionExpression::Not(&in_expression);

    assert_ne!(in_expression, not_in_expression);
}

#[test]
fn condition_expression_not_equal() {
    let equal_expression = ConditionExpression::Condition(Condition {
        field: "status",
        operator: ConditionOperator::Equal,
        value: Value::String("active"),
    });

    let expression = ConditionExpression::Not(&equal_expression);

    match expression {
        ConditionExpression::Not(inner) => {
            assert_eq!(*inner, equal_expression);
        }
        _ => panic!("expected ConditionExpression::Not"),
    }
}

#[test]
fn condition_expression_not_greater_than() {
    let greater_than_expression = ConditionExpression::Condition(Condition {
        field: "size",
        operator: ConditionOperator::GreaterThan,
        value: Value::Uint64(1024),
    });

    let expression = ConditionExpression::Not(&greater_than_expression);

    match expression {
        ConditionExpression::Not(inner) => {
            assert_eq!(*inner, greater_than_expression);
        }
        _ => panic!("expected ConditionExpression::Not"),
    }
}

#[test]
fn condition_expression_not_less_than() {
    let less_than_expression = ConditionExpression::Condition(Condition {
        field: "size",
        operator: ConditionOperator::LessThan,
        value: Value::Uint64(1024),
    });

    let expression = ConditionExpression::Not(&less_than_expression);

    match expression {
        ConditionExpression::Not(inner) => {
            assert_eq!(*inner, less_than_expression);
        }
        _ => panic!("expected ConditionExpression::Not"),
    }
}

#[test]
fn condition_expression_not_contains() {
    let contains_expression = ConditionExpression::Condition(Condition {
        field: "name",
        operator: ConditionOperator::Contains,
        value: Value::String("config"),
    });

    let expression = ConditionExpression::Not(&contains_expression);

    match expression {
        ConditionExpression::Not(inner) => {
            assert_eq!(*inner, contains_expression);
        }
        _ => panic!("expected ConditionExpression::Not"),
    }
}

#[test]
fn condition_expression_not_starts_with() {
    let starts_with_expression = ConditionExpression::Condition(Condition {
        field: "name",
        operator: ConditionOperator::StartsWith,
        value: Value::String("config"),
    });

    let expression = ConditionExpression::Not(&starts_with_expression);

    match expression {
        ConditionExpression::Not(inner) => {
            assert_eq!(*inner, starts_with_expression);
        }
        _ => panic!("expected ConditionExpression::Not"),
    }
}

#[test]
fn condition_expression_not_ends_with() {
    let ends_with_expression = ConditionExpression::Condition(Condition {
        field: "name",
        operator: ConditionOperator::EndsWith,
        value: Value::String(".tmp"),
    });

    let expression = ConditionExpression::Not(&ends_with_expression);

    match expression {
        ConditionExpression::Not(inner) => {
            assert_eq!(*inner, ends_with_expression);
        }
        _ => panic!("expected ConditionExpression::Not"),
    }
}

#[test]
fn condition_expression_not_between() {
    let between_expression = ConditionExpression::Between(BetweenCondition {
        field: "age",
        lower: Value::Uint64(18),
        upper: Value::Uint64(65),
    });

    let expression = ConditionExpression::Not(&between_expression);

    match expression {
        ConditionExpression::Not(inner) => {
            assert_eq!(*inner, between_expression);
        }
        _ => panic!("expected ConditionExpression::Not"),
    }
}

#[test]
fn condition_expression_not_and() {
    let a = ConditionExpression::Condition(Condition {
        field: "name",
        operator: ConditionOperator::Equal,
        value: Value::String("config.evo"),
    });

    let b = ConditionExpression::Condition(Condition {
        field: "size",
        operator: ConditionOperator::GreaterThan,
        value: Value::Uint64(1024),
    });

    let children = [a, b];
    let and_expression = ConditionExpression::And(&children);
    let expression = ConditionExpression::Not(&and_expression);

    match expression {
        ConditionExpression::Not(inner) => {
            assert_eq!(*inner, and_expression);
        }
        _ => panic!("expected ConditionExpression::Not"),
    }
}

#[test]
fn condition_expression_not_or() {
    let a = ConditionExpression::Condition(Condition {
        field: "name",
        operator: ConditionOperator::Equal,
        value: Value::String("config.evo"),
    });

    let b = ConditionExpression::Condition(Condition {
        field: "size",
        operator: ConditionOperator::GreaterThan,
        value: Value::Uint64(1024),
    });

    let children = [a, b];
    let or_expression = ConditionExpression::Or(&children);
    let expression = ConditionExpression::Not(&or_expression);

    match expression {
        ConditionExpression::Not(inner) => {
            assert_eq!(*inner, or_expression);
        }
        _ => panic!("expected ConditionExpression::Not"),
    }
}

#[test]
fn condition_expression_not_clone() {
    let inner = ConditionExpression::Condition(Condition {
        field: "active",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    });

    let original = ConditionExpression::Not(&inner);
    let copied = original.clone();

    assert_eq!(original, copied);
}

#[test]
fn condition_expression_and() {
    let a = ConditionExpression::Condition(Condition {
        field: "name",
        operator: ConditionOperator::Equal,
        value: Value::String("config.evo"),
    });

    let b = ConditionExpression::Condition(Condition {
        field: "size",
        operator: ConditionOperator::GreaterThan,
        value: Value::Uint64(1024),
    });

    let children = [a.clone(), b.clone()];
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
        lower: Value::Uint64(18),
        upper: Value::Uint64(65),
    });

    let condition_expression = ConditionExpression::Condition(Condition {
        field: "active",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    });

    let children = [between_expression.clone(), condition_expression.clone()];
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
fn condition_expression_in_and_condition() {
    let values = [Value::String("active"), Value::String("pending")];
    let in_expression = ConditionExpression::In(InCondition {
        field: "status",
        values: &values,
    });

    let condition_expression = ConditionExpression::Condition(Condition {
        field: "enabled",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    });

    let children = [in_expression.clone(), condition_expression.clone()];
    let expression = ConditionExpression::And(&children);

    match expression {
        ConditionExpression::And(items) => {
            assert_eq!(items.len(), 2);
            assert_eq!(items[0], in_expression);
            assert_eq!(items[1], condition_expression);
            match items[0] {
                ConditionExpression::In(_) => {}
                _ => panic!("expected ConditionExpression::In as first child"),
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
        value: Value::String("config.evo"),
    });

    let b = ConditionExpression::Condition(Condition {
        field: "size",
        operator: ConditionOperator::GreaterThan,
        value: Value::Uint64(1024),
    });

    let children = [a.clone(), b.clone()];
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
        lower: Value::Uint64(50),
        upper: Value::Uint64(100),
    });

    let condition_expression = ConditionExpression::Condition(Condition {
        field: "active",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    });

    let children = [between_expression.clone(), condition_expression.clone()];
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
fn condition_expression_not_in_or_between() {
    let values = [Value::String(".tmp"), Value::String(".bak")];
    let in_expression = ConditionExpression::In(InCondition {
        field: "extension",
        values: &values,
    });
    let not_in_expression = ConditionExpression::Not(&in_expression);

    let between_expression = ConditionExpression::Between(BetweenCondition {
        field: "age",
        lower: Value::Uint64(18),
        upper: Value::Uint64(65),
    });

    let children = [not_in_expression.clone(), between_expression.clone()];
    let expression = ConditionExpression::Or(&children);

    match expression {
        ConditionExpression::Or(items) => {
            assert_eq!(items.len(), 2);
            assert_eq!(items[0], not_in_expression);
            assert_eq!(items[1], between_expression);
        }
        _ => panic!("expected ConditionExpression::Or"),
    }
}

#[test]
fn condition_expression_and_or_grouping() {
    let a = ConditionExpression::Condition(Condition {
        field: "name",
        operator: ConditionOperator::Equal,
        value: Value::String("config.evo"),
    });

    let b = ConditionExpression::Condition(Condition {
        field: "size",
        operator: ConditionOperator::GreaterThan,
        value: Value::Uint64(1024),
    });

    let c = ConditionExpression::Condition(Condition {
        field: "active",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    });

    let and_children = [a.clone(), b.clone()];
    let and_expression = ConditionExpression::And(&and_children);

    let or_children = [and_expression, c.clone()];
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
        lower: Value::Uint64(18),
        upper: Value::Uint64(65),
    });

    let active = ConditionExpression::Condition(Condition {
        field: "active",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    });

    let score = ConditionExpression::Condition(Condition {
        field: "score",
        operator: ConditionOperator::GreaterThan,
        value: Value::Uint64(100),
    });

    let and_children = [between.clone(), active.clone()];
    let and_expression = ConditionExpression::And(&and_children);

    let or_children = [and_expression, score.clone()];
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
fn condition_expression_membership_grouping() {
    let values_in = [Value::String("active"), Value::String("pending")];
    let in_expression = ConditionExpression::In(InCondition {
        field: "status",
        values: &values_in,
    });

    let condition = ConditionExpression::Condition(Condition {
        field: "enabled",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    });

    let values_not_in = [Value::String(".tmp"), Value::String(".bak")];
    let in_for_not = ConditionExpression::In(InCondition {
        field: "extension",
        values: &values_not_in,
    });
    let not_in_expression = ConditionExpression::Not(&in_for_not);

    let and_children = [in_expression.clone(), condition.clone()];
    let and_expression = ConditionExpression::And(&and_children);

    let or_children = [and_expression, not_in_expression.clone()];
    let expression = ConditionExpression::Or(&or_children);

    match expression {
        ConditionExpression::Or(items) => {
            assert_eq!(items.len(), 2);
            match items[0] {
                ConditionExpression::And(and_items) => {
                    assert_eq!(and_items.len(), 2);
                    assert_eq!(and_items[0], in_expression);
                    assert_eq!(and_items[1], condition);
                }
                _ => panic!("expected ConditionExpression::And as first child"),
            }
            assert_eq!(items[1], not_in_expression);
        }
        _ => panic!("expected ConditionExpression::Or as root"),
    }
}

#[test]
fn condition_expression_or_and_grouping() {
    let a = ConditionExpression::Condition(Condition {
        field: "name",
        operator: ConditionOperator::Equal,
        value: Value::String("config.evo"),
    });

    let b = ConditionExpression::Condition(Condition {
        field: "size",
        operator: ConditionOperator::GreaterThan,
        value: Value::Uint64(1024),
    });

    let c = ConditionExpression::Condition(Condition {
        field: "active",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    });

    let or_children = [b.clone(), c.clone()];
    let or_expression = ConditionExpression::Or(&or_children);

    let and_children = [a.clone(), or_expression];
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
        lower: Value::Uint64(18),
        upper: Value::Uint64(65),
    });

    let active = ConditionExpression::Condition(Condition {
        field: "active",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    });

    let score = ConditionExpression::Condition(Condition {
        field: "score",
        operator: ConditionOperator::GreaterThan,
        value: Value::Uint64(100),
    });

    let and_children = [between.clone(), active.clone()];
    let and_expression = ConditionExpression::And(&and_children);
    let or_children_for_first = [and_expression, score.clone()];
    let expression_one = ConditionExpression::Or(&or_children_for_first);

    let or_children_for_second = [active, score];
    let or_expression = ConditionExpression::Or(&or_children_for_second);
    let and_children_for_second = [between, or_expression];
    let expression_two = ConditionExpression::And(&and_children_for_second);

    assert_ne!(expression_one, expression_two);
}

#[test]
fn condition_expression_membership_grouping_inequality() {
    let values_in = [Value::String("active"), Value::String("pending")];
    let in_expression = ConditionExpression::In(InCondition {
        field: "status",
        values: &values_in,
    });

    let condition = ConditionExpression::Condition(Condition {
        field: "enabled",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    });

    let values_not_in = [Value::String(".tmp"), Value::String(".bak")];
    let in_for_not = ConditionExpression::In(InCondition {
        field: "extension",
        values: &values_not_in,
    });
    let not_in_expression = ConditionExpression::Not(&in_for_not);

    let and_children = [in_expression.clone(), condition.clone()];
    let and_expression = ConditionExpression::And(&and_children);
    let or_children_for_first = [and_expression, not_in_expression.clone()];
    let expression_one = ConditionExpression::Or(&or_children_for_first);

    let or_children_for_second = [condition, not_in_expression];
    let or_expression = ConditionExpression::Or(&or_children_for_second);
    let and_children_for_second = [in_expression, or_expression];
    let expression_two = ConditionExpression::And(&and_children_for_second);

    assert_ne!(expression_one, expression_two);
}

#[test]
fn condition_expression_grouping_inequality() {
    let a = ConditionExpression::Condition(Condition {
        field: "name",
        operator: ConditionOperator::Equal,
        value: Value::String("config.evo"),
    });

    let b = ConditionExpression::Condition(Condition {
        field: "size",
        operator: ConditionOperator::GreaterThan,
        value: Value::Uint64(1024),
    });

    let c = ConditionExpression::Condition(Condition {
        field: "active",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    });

    let and_children = [a.clone(), b.clone()];
    let and_expression = ConditionExpression::And(&and_children);
    let or_children_for_first = [and_expression, c.clone()];
    let expression_one = ConditionExpression::Or(&or_children_for_first);

    let or_children_for_second = [b, c];
    let or_expression = ConditionExpression::Or(&or_children_for_second);
    let and_children_for_second = [a, or_expression];
    let expression_two = ConditionExpression::And(&and_children_for_second);

    assert_ne!(expression_one, expression_two);
}

#[test]
fn condition_expression_clone() {
    let condition = Condition {
        field: "active",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    };

    let original = ConditionExpression::Condition(condition);
    let copied = original.clone();

    assert_eq!(original, copied);
}

#[test]
fn condition_expression_between_clone() {
    let between = BetweenCondition {
        field: "age",
        lower: Value::Uint64(18),
        upper: Value::Uint64(65),
    };

    let original = ConditionExpression::Between(between);
    let copied = original.clone();

    assert_eq!(original, copied);
}

#[test]
fn condition_expression_membership_clone() {
    let values = [Value::String("active"), Value::String("pending")];
    let in_condition = InCondition {
        field: "status",
        values: &values,
    };

    let original_in = ConditionExpression::In(in_condition);
    let copied_in = original_in.clone();
    assert_eq!(original_in, copied_in);

    let original_not_in = ConditionExpression::Not(&original_in);
    let copied_not_in = original_not_in.clone();
    assert_eq!(original_not_in, copied_not_in);
}
