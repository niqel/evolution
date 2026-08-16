use evo_shell::definitions::structs::borrowed::condition::Condition;
use evo_shell::definitions::structs::borrowed::value::Value;
use evo_shell::definitions::structs::owned::condition_operator::ConditionOperator;

#[test]
fn condition_text_equality() {
    let condition = Condition {
        field: "name",
        operator: ConditionOperator::Equal,
        value: Value::Text("config.evo"),
    };

    assert_eq!(condition.field, "name");
    assert_eq!(condition.operator, ConditionOperator::Equal);
    assert_eq!(condition.value, Value::Text("config.evo"));
}

#[test]
fn condition_numeric_comparison() {
    let condition = Condition {
        field: "size",
        operator: ConditionOperator::GreaterThan,
        value: Value::Unsigned(1024),
    };

    assert_eq!(condition.field, "size");
    assert_eq!(condition.operator, ConditionOperator::GreaterThan);
    assert_eq!(condition.value, Value::Unsigned(1024));
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
        operator: ConditionOperator::NotEqual,
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
fn condition_copy() {
    let original = Condition {
        field: "size",
        operator: ConditionOperator::LessThanOrEqual,
        value: Value::Unsigned(4096),
    };

    let copied = original;

    assert_eq!(original, copied);
}
