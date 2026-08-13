use evo_shell::definitions::requesters::record_requester;
use evo_shell::definitions::structs::borrowed::field::Field;
use evo_shell::definitions::structs::borrowed::record::Record;
use evo_shell::definitions::structs::borrowed::value::Value;
use evo_shell::definitions::structs::flow::Flow;

fn continue_after_record(record: Record<'_>) -> Flow {
    assert_eq!(record.fields.len(), 2);
    assert_eq!(record.fields[0].name, "name");
    assert_eq!(record.fields[0].value, Value::Text("hola.txt"));
    assert_eq!(record.fields[1].name, "size");
    assert_eq!(record.fields[1].value, Value::Unsigned(20));

    Flow::Continue
}

fn stop_after_record(record: Record<'_>) -> Flow {
    assert_eq!(record.fields.len(), 1);
    assert_eq!(record.fields[0].name, "name_with_prefix");
    assert_eq!(record.fields[0].value, Value::Text("evo_hola.txt"));

    Flow::Stop
}

#[test]
fn record_requester_returns_flow_continue() {
    let request: record_requester::Request = continue_after_record;

    let fields = [
        Field {
            name: "name",
            value: Value::Text("hola.txt"),
        },
        Field {
            name: "size",
            value: Value::Unsigned(20),
        },
    ];

    let record = Record { fields: &fields };

    let flow = request(record);
    assert_eq!(flow, Flow::Continue);
}

#[test]
fn record_requester_returns_flow_stop() {
    let request: record_requester::Request = stop_after_record;

    let fields = [Field {
        name: "name_with_prefix",
        value: Value::Text("evo_hola.txt"),
    }];

    let record = Record { fields: &fields };

    let flow = request(record);
    assert_eq!(flow, Flow::Stop);
}
