use evo_query::definitions::structs::borrowed::iteration_operation::IterationOperation;
use evo_query::definitions::structs::borrowed::new_field::NewField;
use evo_query::definitions::structs::borrowed::selection::Selection;
use evo_query::definitions::structs::borrowed::value_expression::ValueExpression;
use evo_values::definitions::value::Value;

#[test]
fn new_field_literal() {
    let field = NewField {
        name: "fixed_label",
        expression: ValueExpression::Literal(Value::String("system")),
    };

    assert_eq!(field.name, "fixed_label");
    assert_eq!(
        field.expression,
        ValueExpression::Literal(Value::String("system"))
    );
}

#[test]
fn new_field_pipeline() {
    let selections = [Selection::Field("name")];
    let operations = [
        IterationOperation::Select(&selections),
        IterationOperation::ToValue,
    ];

    let field = NewField {
        name: "extracted_name",
        expression: ValueExpression::Pipeline(&operations),
    };

    assert_eq!(field.name, "extracted_name");
    match field.expression {
        ValueExpression::Pipeline(ops) => {
            assert_eq!(ops.len(), 2);
            assert_eq!(ops[0], IterationOperation::Select(&selections));
            assert_eq!(ops[1], IterationOperation::ToValue);
        }
        _ => panic!("expected ValueExpression::Pipeline"),
    }
}

#[test]
fn new_field_concat() {
    let parts = [
        ValueExpression::Literal(Value::String("hello")),
        ValueExpression::Literal(Value::String(" ")),
        ValueExpression::Literal(Value::String("world")),
    ];

    let field = NewField {
        name: "greeting",
        expression: ValueExpression::Concat(&parts),
    };

    assert_eq!(field.name, "greeting");
    match field.expression {
        ValueExpression::Concat(args) => {
            assert_eq!(args.len(), 3);
            assert_eq!(args[0], ValueExpression::Literal(Value::String("hello")));
            assert_eq!(args[1], ValueExpression::Literal(Value::String(" ")));
            assert_eq!(args[2], ValueExpression::Literal(Value::String("world")));
        }
        _ => panic!("expected ValueExpression::Concat"),
    }
}

#[test]
fn new_field_equality_and_difference() {
    let left = NewField {
        name: "tag",
        expression: ValueExpression::Literal(Value::String("prod")),
    };
    let right = NewField {
        name: "tag",
        expression: ValueExpression::Literal(Value::String("prod")),
    };
    let different_name = NewField {
        name: "env",
        expression: ValueExpression::Literal(Value::String("prod")),
    };
    let different_expr = NewField {
        name: "tag",
        expression: ValueExpression::Literal(Value::String("dev")),
    };

    assert_eq!(left, right);
    assert_ne!(left, different_name);
    assert_ne!(left, different_expr);
}

#[test]
fn new_field_clone() {
    let original = NewField {
        name: "tag",
        expression: ValueExpression::Literal(Value::String("prod")),
    };
    let copied = original.clone();

    assert_eq!(original, copied);
}
