use evo_query::definitions::structs::borrowed::field::Field;
use evo_values::definitions::value::Value;

#[test]
fn field_native_and_projected_creation() {
    let native_field = Field {
        name: "name",
        value: Value::String("hola.txt"),
    };

    assert_eq!(native_field.name, "name");
    assert_eq!(native_field.value, Value::String("hola.txt"));

    let projected_field = Field {
        name: "name_with_prefix",
        value: Value::String("evo_hola.txt"),
    };

    assert_eq!(projected_field.name, "name_with_prefix");
    assert_eq!(projected_field.value, Value::String("evo_hola.txt"));
}
