use evo_query::definitions::requesters::construction_requester;
use evo_query::definitions::structs::borrowed::construction::Construction;
use evo_query::definitions::structs::borrowed::field::Field;
use evo_query::definitions::structs::borrowed::record::Record;
use evo_query::definitions::structs::owned::flow::Flow;
use evo_values::definitions::value::Value;

fn continue_after_value(construction: Construction<'_>) -> Flow {
    match construction {
        Construction::Value(Value::Uint64(42)) => Flow::Continue,
        _ => Flow::Stop,
    }
}

fn continue_after_record(construction: Construction<'_>) -> Flow {
    match construction {
        Construction::Record(record) => {
            assert_eq!(record.fields.len(), 1);
            assert_eq!(record.fields[0].name, "name");
            assert_eq!(record.fields[0].value, Value::String("report.txt"));
            Flow::Continue
        }
        _ => Flow::Stop,
    }
}

fn stop_after_construction(_construction: Construction<'_>) -> Flow {
    Flow::Stop
}

#[test]
fn construction_requester_receives_value_and_returns_flow_continue() {
    let request: construction_requester::Request = continue_after_value;

    let flow = request(Construction::Value(Value::Uint64(42)));
    assert_eq!(flow, Flow::Continue);
}

#[test]
fn construction_requester_receives_record_and_returns_flow_continue() {
    let request: construction_requester::Request = continue_after_record;

    let fields = [Field {
        name: "name",
        value: Value::String("report.txt"),
    }];

    let record = Record { fields: &fields };

    let flow = request(Construction::Record(record));
    assert_eq!(flow, Flow::Continue);
}

#[test]
fn construction_requester_returns_flow_stop() {
    let request: construction_requester::Request = stop_after_construction;

    let flow = request(Construction::Value(Value::Boolean(false)));
    assert_eq!(flow, Flow::Stop);
}
