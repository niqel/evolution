use evo_query::definitions::structs::borrowed::condition::Condition;
use evo_query::definitions::structs::owned::condition_operator::ConditionOperator;
use evo_values::definitions::value::Value;

#[test]
fn condition_text_equality() {
    let condition = Condition {
        field: "name",
        operator: ConditionOperator::Equal,
        value: Value::String("config.evo"),
    };

    assert_eq!(condition.field, "name");
    assert_eq!(condition.operator, ConditionOperator::Equal);
    assert_eq!(condition.value, Value::String("config.evo"));
}

#[test]
fn condition_numeric_comparison() {
    let condition = Condition {
        field: "size",
        operator: ConditionOperator::GreaterThan,
        value: Value::Uint64(1024),
    };

    assert_eq!(condition.field, "size");
    assert_eq!(condition.operator, ConditionOperator::GreaterThan);
    assert_eq!(condition.value, Value::Uint64(1024));
}

#[test]
fn condition_equality_and_inequality() {
    let left = Condition {
        field: "active",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    };

    let right = Condition {
        field: "active",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    };

    let different_field = Condition {
        field: "enabled",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(true),
    };

    let different_operator = Condition {
        field: "active",
        operator: ConditionOperator::GreaterThan,
        value: Value::Boolean(true),
    };

    let different_value = Condition {
        field: "active",
        operator: ConditionOperator::Equal,
        value: Value::Boolean(false),
    };

    assert_eq!(left, right);
    assert_ne!(left, different_field);
    assert_ne!(left, different_operator);
    assert_ne!(left, different_value);
}

#[test]
fn condition_clone() {
    let original = Condition {
        field: "size",
        operator: ConditionOperator::LessThan,
        value: Value::Uint64(4096),
    };

    let copied = original.clone();

    assert_eq!(original, copied);
}
