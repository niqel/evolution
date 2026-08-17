use evo_query::definitions::structs::borrowed::new_field::NewField;
use evo_query::definitions::structs::borrowed::selection::Selection;
use evo_query::definitions::structs::borrowed::value_expression::ValueExpression;
use evo_values::definitions::value::Value;

#[test]
fn selection_field() {
    let selection = Selection::Field("name");

    assert_eq!(selection, Selection::Field("name"));

    match selection {
        Selection::Field(name) => assert_eq!(name, "name"),
        _ => panic!("expected Selection::Field"),
    }
}

#[test]
fn selection_new() {
    let new_field = NewField {
        name: "greeting",
        expression: ValueExpression::Literal(Value::Text("hello")),
    };

    let selection = Selection::New(new_field);

    assert_eq!(selection, Selection::New(new_field));

    match selection {
        Selection::New(inner) => {
            assert_eq!(inner.name, "greeting");
            assert_eq!(
                inner.expression,
                ValueExpression::Literal(Value::Text("hello"))
            );
        }
        _ => panic!("expected Selection::New"),
    }
}

#[test]
fn selection_equality_and_difference() {
    let field_a = Selection::Field("name");
    let field_b = Selection::Field("name");
    let field_c = Selection::Field("size");

    let new_a = Selection::New(NewField {
        name: "total",
        expression: ValueExpression::Literal(Value::Unsigned(100)),
    });
    let new_b = Selection::New(NewField {
        name: "total",
        expression: ValueExpression::Literal(Value::Unsigned(100)),
    });
    let new_c = Selection::New(NewField {
        name: "count",
        expression: ValueExpression::Literal(Value::Unsigned(100)),
    });

    assert_eq!(field_a, field_b);
    assert_eq!(new_a, new_b);

    assert_ne!(field_a, field_c);
    assert_ne!(new_a, new_c);
    assert_ne!(field_a, new_a);
}

#[test]
fn selection_copy() {
    let original = Selection::Field("name");
    let copied = original;

    assert_eq!(original, copied);

    let original_new = Selection::New(NewField {
        name: "status",
        expression: ValueExpression::Literal(Value::Boolean(true)),
    });
    let copied_new = original_new;

    assert_eq!(original_new, copied_new);
}
