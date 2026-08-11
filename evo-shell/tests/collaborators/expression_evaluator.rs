use evo_shell::collaborators::arithmetic;
use evo_shell::collaborators::expression_evaluator::{self, Error};
use evo_shell::definitions::structs::borrowed::number_binding::NumberBinding;
use evo_shell::definitions::types::number::Number;

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
        expression_evaluator::evaluate_static("20 / 5 * 2"),
        Ok(Number::I32(8))
    );
    assert_eq!(
        expression_evaluator::evaluate_static("10 - 3 + 2"),
        Ok(Number::I32(9))
    );
}

#[test]
fn number_bindings_evaluation() {
    let bindings1 = [
        NumberBinding {
            name: "quantity",
            value: Number::I32(10),
        },
        NumberBinding {
            name: "price",
            value: Number::I32(5),
        },
        NumberBinding {
            name: "tax",
            value: Number::I32(2),
        },
    ];

    assert_eq!(
        expression_evaluator::evaluate("quantity * price + tax", &bindings1),
        Ok(Number::I32(52))
    );

    let bindings2 = [
        NumberBinding {
            name: "quantity",
            value: Number::I32(3),
        },
        NumberBinding {
            name: "price",
            value: Number::I32(100),
        },
        NumberBinding {
            name: "tax",
            value: Number::I32(20),
        },
    ];

    assert_eq!(
        expression_evaluator::evaluate("quantity * price + tax", &bindings2),
        Ok(Number::I32(320))
    );
}

#[test]
fn duplicate_binding_first_wins() {
    let bindings = [
        NumberBinding {
            name: "x",
            value: Number::I32(10),
        },
        NumberBinding {
            name: "x",
            value: Number::I32(20),
        },
    ];

    assert_eq!(
        expression_evaluator::evaluate("x", &bindings),
        Ok(Number::I32(10))
    );
}

#[test]
fn precise_error_reporting() {
    assert_eq!(
        expression_evaluator::evaluate_static(""),
        Err(Error::EmptyExpression)
    );
    assert_eq!(
        expression_evaluator::evaluate_static("   "),
        Err(Error::EmptyExpression)
    );
    assert_eq!(
        expression_evaluator::evaluate_static("2 +"),
        Err(Error::UnexpectedEnd)
    );
    assert_eq!(
        expression_evaluator::evaluate_static("2 + * 3"),
        Err(Error::UnexpectedToken)
    );
    assert_eq!(
        expression_evaluator::evaluate_static("* 3"),
        Err(Error::UnexpectedToken)
    );
    assert_eq!(
        expression_evaluator::evaluate_static("(2 + 3"),
        Err(Error::MissingClosingParenthesis)
    );
    assert_eq!(
        expression_evaluator::evaluate_static("2 + (3 * 4"),
        Err(Error::MissingClosingParenthesis)
    );
    assert_eq!(
        expression_evaluator::evaluate_static("2 + 3)"),
        Err(Error::UnexpectedToken)
    );
    assert_eq!(
        expression_evaluator::evaluate_static("unknown"),
        Err(Error::UnknownIdentifier)
    );
    assert_eq!(
        expression_evaluator::evaluate_static("256u8"),
        Err(Error::InvalidNumber)
    );
    assert_eq!(
        expression_evaluator::evaluate_static("128i8"),
        Err(Error::InvalidNumber)
    );
    assert_eq!(
        expression_evaluator::evaluate_static("2147483648"),
        Err(Error::InvalidNumber)
    );
}

#[test]
fn signed_minimum_literals() {
    assert_eq!(
        expression_evaluator::evaluate_static("-128i8"),
        Ok(Number::I8(i8::MIN))
    );
    assert_eq!(
        expression_evaluator::evaluate_static("-32768i16"),
        Ok(Number::I16(i16::MIN))
    );
    assert_eq!(
        expression_evaluator::evaluate_static("-2147483648i32"),
        Ok(Number::I32(i32::MIN))
    );
    assert_eq!(
        expression_evaluator::evaluate_static("-9223372036854775808i64"),
        Ok(Number::I64(i64::MIN))
    );
    assert_eq!(
        expression_evaluator::evaluate_static("-170141183460469231731687303715884105728i128"),
        Ok(Number::I128(i128::MIN))
    );
    assert_eq!(
        expression_evaluator::evaluate_static("-2147483648"),
        Ok(Number::I32(i32::MIN))
    );

    assert_eq!(
        expression_evaluator::evaluate_static("-2147483649"),
        Err(Error::InvalidNumber)
    );
    assert_eq!(
        expression_evaluator::evaluate_static("-129i8"),
        Err(Error::InvalidNumber)
    );
    assert_eq!(
        expression_evaluator::evaluate_static("-32769i16"),
        Err(Error::InvalidNumber)
    );
}

#[test]
fn expression_negate_semantics() {
    assert_eq!(
        expression_evaluator::evaluate_static("-5u8"),
        Err(Error::Arithmetic(arithmetic::Error::UnsupportedTypes))
    );
    assert_eq!(
        expression_evaluator::evaluate_static("-0u8"),
        Err(Error::Arithmetic(arithmetic::Error::UnsupportedTypes))
    );
    assert_eq!(
        expression_evaluator::evaluate_static("-(-128i8)"),
        Err(Error::Arithmetic(arithmetic::Error::Overflow))
    );
    assert_eq!(
        expression_evaluator::evaluate_static("--128i8"),
        Err(Error::Arithmetic(arithmetic::Error::Overflow))
    );
    assert_eq!(
        expression_evaluator::evaluate_static("-(2 + 3)"),
        Ok(Number::I32(-5))
    );
}
