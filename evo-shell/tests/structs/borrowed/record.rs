use evo_shell::definitions::structs::borrowed::field::Field;
use evo_shell::definitions::structs::borrowed::record::Record;
use evo_shell::definitions::structs::borrowed::value::Value;

#[test]
fn record_borrowed_fields() {
    let name = "hola.txt";

    let fields = [
        Field {
            name: "name",
            value: Value::Text(name),
        },
        Field {
            name: "size",
            value: Value::Unsigned(20),
        },
    ];

    let record = Record { fields: &fields };

    assert_eq!(record.fields.len(), 2);
    assert_eq!(record.fields[0].name, "name");
    assert_eq!(record.fields[0].value, Value::Text("hola.txt"));
    assert_eq!(record.fields[1].name, "size");
    assert_eq!(record.fields[1].value, Value::Unsigned(20));
}
