use evo_shell::definitions::structs::borrowed::construction::Construction;
use evo_shell::definitions::structs::borrowed::field::Field;
use evo_shell::definitions::structs::borrowed::record::Record;
use evo_values::definitions::value::Value;

#[test]
fn construction_record_wrapping_and_field_preservation() {
    let fields = [
        Field {
            name: "name",
            value: Value::Text("report.txt"),
        },
        Field {
            name: "size",
            value: Value::Unsigned(8500),
        },
    ];

    let record = Record { fields: &fields };
    let construction = Construction::Record(record);

    match construction {
        Construction::Record(r) => {
            assert_eq!(r.fields.len(), 2);
            assert_eq!(r.fields[0].name, "name");
            assert_eq!(r.fields[0].value, Value::Text("report.txt"));
            assert_eq!(r.fields[1].name, "size");
            assert_eq!(r.fields[1].value, Value::Unsigned(8500));
        }
        Construction::Value(_) => panic!("expected Construction::Record"),
    }
}

#[test]
fn construction_value_wrapping() {
    let unsigned_construction = Construction::Value(Value::Unsigned(3));
    assert_eq!(
        unsigned_construction,
        Construction::Value(Value::Unsigned(3))
    );
    assert_ne!(
        unsigned_construction,
        Construction::Value(Value::Unsigned(4))
    );

    let text_construction = Construction::Value(Value::Text("hello"));
    assert_eq!(text_construction, Construction::Value(Value::Text("hello")));
    assert_ne!(text_construction, Construction::Value(Value::Text("world")));
}
