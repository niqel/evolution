use evo_shell::definitions::structs::borrowed::iteration::Iteration;
use evo_shell::definitions::structs::borrowed::iteration_operation::IterationOperation;

#[test]
fn iteration_ordered_pipeline() {
    let fields = ["name", "size"];
    let operations = [
        IterationOperation::Select(&fields),
        IterationOperation::Skip(20),
        IterationOperation::Take(10),
        IterationOperation::Iter,
    ];

    let iteration = Iteration {
        operations: &operations,
    };

    assert_eq!(iteration.operations.len(), 4);

    match iteration.operations[0] {
        IterationOperation::Select(selected_fields) => {
            assert_eq!(selected_fields.len(), 2);
            assert_eq!(selected_fields[0], "name");
            assert_eq!(selected_fields[1], "size");
        }
        _ => panic!("expected IterationOperation::Select"),
    }

    assert_eq!(iteration.operations[1], IterationOperation::Skip(20));
    assert_eq!(iteration.operations[2], IterationOperation::Take(10));
    assert_eq!(iteration.operations[3], IterationOperation::Iter);
}

#[test]
fn iteration_empty_pipeline() {
    let operations: [IterationOperation<'_>; 0] = [];

    let iteration = Iteration {
        operations: &operations,
    };

    assert!(iteration.operations.is_empty());
}

#[test]
fn iteration_single_operation() {
    let operations = [IterationOperation::Count];

    let iteration = Iteration {
        operations: &operations,
    };

    assert_eq!(iteration.operations.len(), 1);
    assert_eq!(iteration.operations[0], IterationOperation::Count);
}

#[test]
fn iteration_copy() {
    let operations = [IterationOperation::Take(5), IterationOperation::Iter];

    let original = Iteration {
        operations: &operations,
    };

    let copied = original;

    assert_eq!(original, copied);
}
