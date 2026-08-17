use evo_query::definitions::structs::borrowed::between_condition::BetweenCondition;
use evo_values::definitions::value::Value;

#[test]
fn between_condition_unsigned() {
    let condition = BetweenCondition {
        field: "age",
        lower: Value::Unsigned(18),
        upper: Value::Unsigned(65),
    };

    assert_eq!(condition.field, "age");
    assert_eq!(condition.lower, Value::Unsigned(18));
    assert_eq!(condition.upper, Value::Unsigned(65));
}

#[test]
fn between_condition_signed() {
    let condition = BetweenCondition {
        field: "temperature",
        lower: Value::Signed(-10),
        upper: Value::Signed(40),
    };

    assert_eq!(condition.field, "temperature");
    assert_eq!(condition.lower, Value::Signed(-10));
    assert_eq!(condition.upper, Value::Signed(40));
}

#[test]
fn between_condition_text() {
    let condition = BetweenCondition {
        field: "created_date",
        lower: Value::Text("2026-01-01"),
        upper: Value::Text("2026-01-31"),
    };

    assert_eq!(condition.field, "created_date");
    assert_eq!(condition.lower, Value::Text("2026-01-01"));
    assert_eq!(condition.upper, Value::Text("2026-01-31"));
}

#[test]
fn between_condition_equality_and_difference() {
    let left = BetweenCondition {
        field: "age",
        lower: Value::Unsigned(18),
        upper: Value::Unsigned(65),
    };

    let right = BetweenCondition {
        field: "age",
        lower: Value::Unsigned(18),
        upper: Value::Unsigned(65),
    };

    let different_field = BetweenCondition {
        field: "score",
        lower: Value::Unsigned(18),
        upper: Value::Unsigned(65),
    };

    let different_lower = BetweenCondition {
        field: "age",
        lower: Value::Unsigned(21),
        upper: Value::Unsigned(65),
    };

    let different_upper = BetweenCondition {
        field: "age",
        lower: Value::Unsigned(18),
        upper: Value::Unsigned(60),
    };

    assert_eq!(left, right);
    assert_ne!(left, different_field);
    assert_ne!(left, different_lower);
    assert_ne!(left, different_upper);
}

#[test]
fn between_condition_copy() {
    let original = BetweenCondition {
        field: "age",
        lower: Value::Unsigned(18),
        upper: Value::Unsigned(65),
    };

    let copied = original;

    assert_eq!(original, copied);
}
