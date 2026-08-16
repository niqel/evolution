use evo_shell::definitions::structs::borrowed::iteration_operation::IterationOperation;

#[test]
fn iteration_operation_select() {
    let fields = ["name", "size"];
    let operation = IterationOperation::Select(&fields);

    match operation {
        IterationOperation::Select(selected_fields) => {
            assert_eq!(selected_fields.len(), 2);
            assert_eq!(selected_fields[0], "name");
            assert_eq!(selected_fields[1], "size");
        }
        _ => panic!("expected IterationOperation::Select"),
    }
}

#[test]
fn iteration_operation_take() {
    let operation = IterationOperation::Take(10);
    assert_eq!(operation, IterationOperation::Take(10));
    assert_ne!(operation, IterationOperation::Take(5));

    match operation {
        IterationOperation::Take(count) => assert_eq!(count, 10),
        _ => panic!("expected IterationOperation::Take"),
    }
}

#[test]
fn iteration_operation_skip() {
    let operation = IterationOperation::Skip(20);
    assert_eq!(operation, IterationOperation::Skip(20));
    assert_ne!(operation, IterationOperation::Skip(10));

    match operation {
        IterationOperation::Skip(count) => assert_eq!(count, 20),
        _ => panic!("expected IterationOperation::Skip"),
    }
}

#[test]
fn iteration_operation_unit_variants() {
    assert_eq!(IterationOperation::First, IterationOperation::First);
    assert_eq!(IterationOperation::Last, IterationOperation::Last);
    assert_eq!(IterationOperation::Count, IterationOperation::Count);
    assert_eq!(IterationOperation::Iter, IterationOperation::Iter);

    assert_ne!(IterationOperation::First, IterationOperation::Last);
    assert_ne!(IterationOperation::Count, IterationOperation::Iter);
}

#[test]
fn iteration_operation_copy() {
    let fields = ["name"];
    let original = IterationOperation::Select(&fields);
    let copied = original;

    assert_eq!(original, copied);
}
