use evo_query::definitions::structs::borrowed::between_condition::BetweenCondition;
use evo_values::definitions::value::Value;

#[test]
fn between_condition_unsigned() {
    let condition = BetweenCondition {
        field: "age",
        lower: Value::Uint64(18),
        upper: Value::Uint64(65),
    };

    assert_eq!(condition.field, "age");
    assert_eq!(condition.lower, Value::Uint64(18));
    assert_eq!(condition.upper, Value::Uint64(65));
}

#[test]
fn between_condition_signed() {
    let condition = BetweenCondition {
        field: "temperature",
        lower: Value::Int64(-10),
        upper: Value::Int64(40),
    };

    assert_eq!(condition.field, "temperature");
    assert_eq!(condition.lower, Value::Int64(-10));
    assert_eq!(condition.upper, Value::Int64(40));
}

#[test]
fn between_condition_text() {
    let condition = BetweenCondition {
        field: "created_date",
        lower: Value::String("2026-01-01"),
        upper: Value::String("2026-01-31"),
    };

    assert_eq!(condition.field, "created_date");
    assert_eq!(condition.lower, Value::String("2026-01-01"));
    assert_eq!(condition.upper, Value::String("2026-01-31"));
}

#[test]
fn between_condition_equality_and_difference() {
    let left = BetweenCondition {
        field: "age",
        lower: Value::Uint64(18),
        upper: Value::Uint64(65),
    };

    let right = BetweenCondition {
        field: "age",
        lower: Value::Uint64(18),
        upper: Value::Uint64(65),
    };

    let different_field = BetweenCondition {
        field: "score",
        lower: Value::Uint64(18),
        upper: Value::Uint64(65),
    };

    let different_lower = BetweenCondition {
        field: "age",
        lower: Value::Uint64(21),
        upper: Value::Uint64(65),
    };

    let different_upper = BetweenCondition {
        field: "age",
        lower: Value::Uint64(18),
        upper: Value::Uint64(60),
    };

    assert_eq!(left, right);
    assert_ne!(left, different_field);
    assert_ne!(left, different_lower);
    assert_ne!(left, different_upper);
}

#[test]
fn between_condition_clone() {
    let original = BetweenCondition {
        field: "age",
        lower: Value::Uint64(18),
        upper: Value::Uint64(65),
    };

    let copied = original.clone();

    assert_eq!(original, copied);
}
