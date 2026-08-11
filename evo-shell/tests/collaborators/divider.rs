use evo_shell::collaborators::divider;
use evo_shell::definitions::types::number::Number;
use evo_shell::definitions::use_cases::divide::Error;

#[test]
fn division_examples() {
    assert_eq!(
        divider::collaborate(Number::I32(7), Number::I32(3)),
        Ok(Number::I32(2))
    );
    assert_eq!(
        divider::collaborate(Number::I32(-7), Number::I32(3)),
        Ok(Number::I32(-2))
    );
    assert_eq!(
        divider::collaborate(Number::F64(10.0), Number::I32(4)),
        Ok(Number::F64(2.5))
    );
    assert_eq!(
        divider::collaborate(Number::I32(10), Number::F64(4.0)),
        Ok(Number::F64(2.5))
    );
}

#[test]
fn division_by_zero_and_overflow_errors() {
    assert_eq!(
        divider::collaborate(Number::I32(10), Number::I32(0)),
        Err(Error::DivisionByZero)
    );
    assert_eq!(
        divider::collaborate(Number::I8(i8::MIN), Number::I8(-1)),
        Err(Error::Overflow)
    );
}

#[test]
fn float_ieee754_behavior() {
    let div_inf = divider::collaborate(Number::F64(1.0), Number::F64(0.0));
    assert_eq!(div_inf, Ok(Number::F64(f64::INFINITY)));

    let div_nan = divider::collaborate(Number::F64(0.0), Number::F64(0.0));
    match div_nan {
        Ok(Number::F64(val)) => assert!(val.is_nan()),
        _ => panic!("Expected F64(NaN)"),
    }
}
