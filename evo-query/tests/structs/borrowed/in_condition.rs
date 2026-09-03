use evo_query::definitions::structs::borrowed::in_condition::InCondition;
use evo_values::definitions::value::Value;

#[test]
fn in_condition_text_values() {
    let values = [
        Value::String("active"),
        Value::String("pending"),
        Value::String("review"),
    ];

    let condition = InCondition {
        field: "status",
        values: &values,
    };

    assert_eq!(condition.field, "status");
    assert_eq!(condition.values.len(), 3);
    assert_eq!(condition.values[0], Value::String("active"));
    assert_eq!(condition.values[1], Value::String("pending"));
    assert_eq!(condition.values[2], Value::String("review"));
}

#[test]
fn in_condition_unsigned_values() {
    let values = [Value::Uint64(10), Value::Uint64(25), Value::Uint64(40)];

    let condition = InCondition {
        field: "category_id",
        values: &values,
    };

    assert_eq!(condition.field, "category_id");
    assert_eq!(condition.values.len(), 3);
    assert_eq!(condition.values[0], Value::Uint64(10));
    assert_eq!(condition.values[1], Value::Uint64(25));
    assert_eq!(condition.values[2], Value::Uint64(40));
}

#[test]
fn in_condition_borrows_values() {
    let values = [Value::String("admin"), Value::String("guest")];

    let condition = InCondition {
        field: "role",
        values: &values,
    };

    assert_eq!(condition.values.as_ptr(), values.as_ptr());
    assert_eq!(condition.values.len(), 2);
    assert_eq!(condition.values[0], Value::String("admin"));
    assert_eq!(condition.values[1], Value::String("guest"));
}

#[test]
fn in_condition_equality_and_difference() {
    let values_left = [Value::String("active"), Value::String("pending")];
    let values_right = [Value::String("active"), Value::String("pending")];
    let values_different = [Value::String("inactive"), Value::String("pending")];

    let left = InCondition {
        field: "status",
        values: &values_left,
    };

    let right = InCondition {
        field: "status",
        values: &values_right,
    };

    let different_field = InCondition {
        field: "state",
        values: &values_left,
    };

    let different_values = InCondition {
        field: "status",
        values: &values_different,
    };

    assert_eq!(left, right);
    assert_ne!(left, different_field);
    assert_ne!(left, different_values);
}

#[test]
fn in_condition_copy() {
    let values = [Value::Uint64(1), Value::Uint64(2)];

    let original = InCondition {
        field: "priority",
        values: &values,
    };

    let copied = original;

    assert_eq!(original, copied);
}

#[test]
fn in_condition_empty_values_is_representable() {
    let values: [Value<'_>; 0] = [];

    let condition = InCondition {
        field: "status",
        values: &values,
    };

    assert!(condition.values.is_empty());
}
