use evo_shell::definitions::structs::borrowed::value::Value;

#[test]
fn value_variants_and_equality() {
    assert_eq!(Value::Text("hola"), Value::Text("hola"));
    assert_ne!(Value::Text("hola"), Value::Text("mundo"));

    assert_eq!(Value::Unsigned(20), Value::Unsigned(20));
    assert_ne!(Value::Unsigned(20), Value::Unsigned(10));

    assert_eq!(Value::Signed(-20), Value::Signed(-20));
    assert_ne!(Value::Signed(-20), Value::Signed(20));

    assert_eq!(Value::Boolean(true), Value::Boolean(true));
    assert_ne!(Value::Boolean(true), Value::Boolean(false));
}
