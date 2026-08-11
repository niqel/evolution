use evo_shell::collaborators::arithmetic;
use evo_shell::collaborators::expression_evaluator::{self, Error};
use evo_shell::definitions::types::number::Number;

fn mock_variable_resolver(name: &str) -> Result<Number, Error> {
    match name {
        "quantity" => Ok(Number::I32(10)),
        "price" => Ok(Number::I32(5)),
        "tax" => Ok(Number::I32(2)),
        "rate" => Ok(Number::F64(0.15)),
        _ => Err(Error::VariableNotFound),
    }
}

#[test]
fn static_expression_precedence() {
    assert_eq!(
        expression_evaluator::evaluate_static("2 + 3 * 4"),
        Ok(Number::I32(14))
    );
    assert_eq!(
        expression_evaluator::evaluate_static("(2 + 3) * 4"),
        Ok(Number::I32(20))
    );
    assert_eq!(
        expression_evaluator::evaluate_static("10 - 2 * 3"),
        Ok(Number::I32(4))
    );
    assert_eq!(
        expression_evaluator::evaluate_static("(10 - 2) * 3"),
        Ok(Number::I32(24))
    );
}

#[test]
fn unary_operators() {
    assert_eq!(
        expression_evaluator::evaluate_static("-5 + 10"),
        Ok(Number::I32(5))
    );
    assert_eq!(
        expression_evaluator::evaluate_static("-(5 + 10)"),
        Ok(Number::I32(-15))
    );
    assert_eq!(
        expression_evaluator::evaluate_static("+5 + 10"),
        Ok(Number::I32(15))
    );
}

#[test]
fn variable_expressions() {
    assert_eq!(
        expression_evaluator::evaluate("quantity * price + tax", mock_variable_resolver),
        Ok(Number::I32(52))
    );
    assert_eq!(
        expression_evaluator::evaluate("tax + (quantity * price)", mock_variable_resolver),
        Ok(Number::I32(52))
    );
    assert_eq!(
        expression_evaluator::evaluate("(quantity + tax) * price", mock_variable_resolver),
        Ok(Number::I32(60))
    );
}

#[test]
fn float_and_mixed_type_expressions() {
    assert_eq!(
        expression_evaluator::evaluate_static("2.5 * 4.0"),
        Ok(Number::F64(10.0))
    );
    assert_eq!(
        expression_evaluator::evaluate_static("2.5f32 * 4.0f32"),
        Ok(Number::F32(10.0))
    );
    assert_eq!(
        expression_evaluator::evaluate_static("10 + 2.5"),
        Ok(Number::F64(12.5))
    );
}

#[test]
fn syntax_and_parenthesis_errors() {
    assert_eq!(
        expression_evaluator::evaluate_static("(2 + 3"),
        Err(Error::UnmatchedParenthesis)
    );
    assert_eq!(
        expression_evaluator::evaluate_static("2 + 3)"),
        Err(Error::UnmatchedParenthesis)
    );
    assert_eq!(
        expression_evaluator::evaluate_static("2 +"),
        Err(Error::EmptyExpression)
    );
    assert_eq!(
        expression_evaluator::evaluate_static(""),
        Err(Error::EmptyExpression)
    );
    assert_eq!(
        expression_evaluator::evaluate_static("2 + * 3"),
        Err(Error::InvalidSyntax)
    );
}

#[test]
fn variable_not_found_error() {
    assert_eq!(
        expression_evaluator::evaluate("unknown_var", mock_variable_resolver),
        Err(Error::VariableNotFound)
    );
}

#[test]
fn arithmetic_errors_propagated() {
    assert_eq!(
        expression_evaluator::evaluate_static("10 / 0"),
        Err(Error::Arithmetic(arithmetic::Error::DivisionByZero))
    );
    assert_eq!(
        expression_evaluator::evaluate_static("255u8 + 1u8"),
        Err(Error::Arithmetic(arithmetic::Error::Overflow))
    );
}
