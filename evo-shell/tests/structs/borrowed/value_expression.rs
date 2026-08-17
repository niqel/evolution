use evo_shell::definitions::structs::borrowed::iteration_operation::IterationOperation;
use evo_shell::definitions::structs::borrowed::len_expression::LenExpression;
use evo_shell::definitions::structs::borrowed::new_field::NewField;
use evo_shell::definitions::structs::borrowed::replace_expression::ReplaceExpression;
use evo_shell::definitions::structs::borrowed::selection::Selection;
use evo_shell::definitions::structs::borrowed::substring_expression::SubstringExpression;
use evo_shell::definitions::structs::borrowed::value_expression::ValueExpression;
use evo_values::definitions::value::Value;

#[test]
fn value_expression_literal_text() {
    let expr = ValueExpression::Literal(Value::Text("sample"));

    assert_eq!(expr, ValueExpression::Literal(Value::Text("sample")));
    match expr {
        ValueExpression::Literal(Value::Text(text)) => assert_eq!(text, "sample"),
        _ => panic!("expected ValueExpression::Literal(Value::Text)"),
    }
}

#[test]
fn value_expression_literal_unsigned() {
    let expr = ValueExpression::Literal(Value::Unsigned(42));

    assert_eq!(expr, ValueExpression::Literal(Value::Unsigned(42)));
    match expr {
        ValueExpression::Literal(Value::Unsigned(n)) => assert_eq!(n, 42),
        _ => panic!("expected ValueExpression::Literal(Value::Unsigned)"),
    }
}

#[test]
fn value_expression_pipeline() {
    let selections = [Selection::Field("age")];
    let operations = [
        IterationOperation::Select(&selections),
        IterationOperation::ToValue,
    ];

    let expr = ValueExpression::Pipeline(&operations);

    match expr {
        ValueExpression::Pipeline(ops) => {
            assert_eq!(ops.len(), 2);
            assert_eq!(ops[0], IterationOperation::Select(&selections));
            assert_eq!(ops[1], IterationOperation::ToValue);
        }
        _ => panic!("expected ValueExpression::Pipeline"),
    }
}

#[test]
fn value_expression_concat() {
    let args = [
        ValueExpression::Literal(Value::Text("prefix_")),
        ValueExpression::Literal(Value::Text("suffix")),
    ];

    let expr = ValueExpression::Concat(&args);

    match expr {
        ValueExpression::Concat(items) => {
            assert_eq!(items.len(), 2);
            assert_eq!(items[0], ValueExpression::Literal(Value::Text("prefix_")));
            assert_eq!(items[1], ValueExpression::Literal(Value::Text("suffix")));
        }
        _ => panic!("expected ValueExpression::Concat"),
    }
}

#[test]
fn value_expression_concat_preserves_argument_order() {
    let a = ValueExpression::Literal(Value::Text("first"));
    let b = ValueExpression::Literal(Value::Text("second"));
    let c = ValueExpression::Literal(Value::Text("third"));

    let args_in_order = [a, b, c];
    let expr_in_order = ValueExpression::Concat(&args_in_order);

    let args_reversed = [c, b, a];
    let expr_reversed = ValueExpression::Concat(&args_reversed);

    assert_ne!(expr_in_order, expr_reversed);

    match expr_in_order {
        ValueExpression::Concat(items) => {
            assert_eq!(items[0], a);
            assert_eq!(items[1], b);
            assert_eq!(items[2], c);
        }
        _ => panic!("expected ValueExpression::Concat"),
    }
}

#[test]
fn value_expression_copy() {
    let original = ValueExpression::Literal(Value::Text("test"));
    let copied = original;

    assert_eq!(original, copied);
}

#[test]
fn value_expression_inequality() {
    let lit_a = ValueExpression::Literal(Value::Text("a"));
    let lit_b = ValueExpression::Literal(Value::Text("b"));
    let args = [lit_a];
    let concat_a = ValueExpression::Concat(&args);

    assert_ne!(lit_a, lit_b);
    assert_ne!(lit_a, concat_a);
}

#[test]
fn select_new_field_from_concat_expression() {
    let name_selections = [Selection::Field("name")];
    let name_operations = [
        IterationOperation::Select(&name_selections),
        IterationOperation::ToValue,
    ];

    let last_name_selections = [Selection::Field("last_name")];
    let last_name_operations = [
        IterationOperation::Select(&last_name_selections),
        IterationOperation::ToValue,
    ];

    let concat_arguments = [
        ValueExpression::Pipeline(&name_operations),
        ValueExpression::Literal(Value::Text(" ")),
        ValueExpression::Pipeline(&last_name_operations),
    ];

    let selections = [Selection::New(NewField {
        name: "full_name",
        expression: ValueExpression::Concat(&concat_arguments),
    })];

    let operation = IterationOperation::Select(&selections);

    match operation {
        IterationOperation::Select(sel) => {
            assert_eq!(sel.len(), 1);
            match sel[0] {
                Selection::New(ref field) => {
                    assert_eq!(field.name, "full_name");
                    match field.expression {
                        ValueExpression::Concat(args) => {
                            assert_eq!(args.len(), 3);

                            match args[0] {
                                ValueExpression::Pipeline(ops) => {
                                    assert_eq!(ops.len(), 2);
                                    assert_eq!(
                                        ops[0],
                                        IterationOperation::Select(&name_selections)
                                    );
                                    assert_eq!(ops[1], IterationOperation::ToValue);
                                }
                                _ => panic!("expected Pipeline for first argument"),
                            }

                            assert_eq!(args[1], ValueExpression::Literal(Value::Text(" ")));

                            match args[2] {
                                ValueExpression::Pipeline(ops) => {
                                    assert_eq!(ops.len(), 2);
                                    assert_eq!(
                                        ops[0],
                                        IterationOperation::Select(&last_name_selections)
                                    );
                                    assert_eq!(ops[1], IterationOperation::ToValue);
                                }
                                _ => panic!("expected Pipeline for third argument"),
                            }
                        }
                        _ => panic!("expected Concat expression"),
                    }
                }
                _ => panic!("expected Selection::New"),
            }
        }
        _ => panic!("expected IterationOperation::Select"),
    }
}

#[test]
fn value_expression_substring_can_be_constructed() {
    let text = ValueExpression::Literal(Value::Text("México"));
    let start = ValueExpression::Literal(Value::Unsigned(1));
    let length = ValueExpression::Literal(Value::Unsigned(3));
    let substring_expr = SubstringExpression {
        text: &text,
        start: &start,
        length: &length,
    };
    let expr = ValueExpression::Substring(substring_expr);

    assert_eq!(expr, ValueExpression::Substring(substring_expr));
    match expr {
        ValueExpression::Substring(sub) => {
            assert_eq!(*sub.text, text);
            assert_eq!(*sub.start, start);
            assert_eq!(*sub.length, length);
        }
        _ => panic!("expected ValueExpression::Substring"),
    }
}

#[test]
fn value_expression_len_can_be_constructed() {
    let text = ValueExpression::Literal(Value::Text("México"));
    let len_expr = LenExpression { text: &text };
    let expr = ValueExpression::Len(len_expr);

    assert_eq!(expr, ValueExpression::Len(len_expr));
    match expr {
        ValueExpression::Len(len) => {
            assert_eq!(*len.text, text);
        }
        _ => panic!("expected ValueExpression::Len"),
    }
}

#[test]
fn value_expression_replace_can_be_constructed() {
    let text = ValueExpression::Literal(Value::Text("one two one"));
    let from = ValueExpression::Literal(Value::Text("one"));
    let to = ValueExpression::Literal(Value::Text("1"));
    let replace_expr = ReplaceExpression {
        text: &text,
        from: &from,
        to: &to,
    };
    let expr = ValueExpression::Replace(replace_expr);

    assert_eq!(expr, ValueExpression::Replace(replace_expr));
    match expr {
        ValueExpression::Replace(rep) => {
            assert_eq!(*rep.text, text);
            assert_eq!(*rep.from, from);
            assert_eq!(*rep.to, to);
        }
        _ => panic!("expected ValueExpression::Replace"),
    }
}

#[test]
fn value_expressions_can_be_nested() {
    let part1 = ValueExpression::Literal(Value::Text("Gustavo"));
    let part2 = ValueExpression::Literal(Value::Text(" "));
    let part3 = ValueExpression::Literal(Value::Text("Melendez"));
    let parts = [part1, part2, part3];
    let concat_expr = ValueExpression::Concat(&parts);
    let len_expr = LenExpression { text: &concat_expr };
    let nested = ValueExpression::Len(len_expr);

    match nested {
        ValueExpression::Len(len) => match *len.text {
            ValueExpression::Concat(args) => {
                assert_eq!(args.len(), 3);
                assert_eq!(args[0], part1);
                assert_eq!(args[1], part2);
                assert_eq!(args[2], part3);
            }
            _ => panic!("expected nested Concat"),
        },
        _ => panic!("expected Len expression"),
    }
}
