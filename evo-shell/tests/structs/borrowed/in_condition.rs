use evo_shell::definitions::structs::borrowed::in_condition::InCondition;
use evo_values::definitions::value::Value;

#[test]
fn in_condition_text_values() {
    let values = [
        Value::Text("active"),
        Value::Text("pending"),
        Value::Text("review"),
    ];

    let condition = InCondition {
        field: "status",
        values: &values,
    };

    assert_eq!(condition.field, "status");
    assert_eq!(condition.values.len(), 3);
    assert_eq!(condition.values[0], Value::Text("active"));
    assert_eq!(condition.values[1], Value::Text("pending"));
    assert_eq!(condition.values[2], Value::Text("review"));
}

#[test]
fn in_condition_unsigned_values() {
    let values = [
        Value::Unsigned(10),
        Value::Unsigned(25),
        Value::Unsigned(40),
    ];

    let condition = InCondition {
        field: "category_id",
        values: &values,
    };

    assert_eq!(condition.field, "category_id");
    assert_eq!(condition.values.len(), 3);
    assert_eq!(condition.values[0], Value::Unsigned(10));
    assert_eq!(condition.values[1], Value::Unsigned(25));
    assert_eq!(condition.values[2], Value::Unsigned(40));
}

#[test]
fn in_condition_borrows_values() {
    let values = [Value::Text("admin"), Value::Text("guest")];

    let condition = InCondition {
        field: "role",
        values: &values,
    };

    assert_eq!(condition.values.as_ptr(), values.as_ptr());
    assert_eq!(condition.values.len(), 2);
    assert_eq!(condition.values[0], Value::Text("admin"));
    assert_eq!(condition.values[1], Value::Text("guest"));
}

#[test]
fn in_condition_equality_and_difference() {
    let values_left = [Value::Text("active"), Value::Text("pending")];
    let values_right = [Value::Text("active"), Value::Text("pending")];
    let values_different = [Value::Text("inactive"), Value::Text("pending")];

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
    let values = [Value::Unsigned(1), Value::Unsigned(2)];

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
