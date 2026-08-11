use evo_shell::collaborators::arithmetic::{self, Error};
use evo_shell::definitions::types::number::Number;

#[test]
fn remainder_examples() {
    assert_eq!(
        arithmetic::remainder(Number::I32(7), Number::I32(3)),
        Ok(Number::I32(1))
    );
    assert_eq!(
        arithmetic::remainder(Number::I32(-7), Number::I32(3)),
        Ok(Number::I32(-1))
    );
    assert_eq!(
        arithmetic::remainder(Number::U8(7), Number::U8(3)),
        Ok(Number::U8(1))
    );
}

#[test]
fn remainder_errors() {
    assert_eq!(
        arithmetic::remainder(Number::I32(10), Number::I32(0)),
        Err(Error::DivisionByZero)
    );
    assert_eq!(
        arithmetic::remainder(Number::F64(7.0), Number::F64(3.0)),
        Err(Error::UnsupportedTypes)
    );
}

#[test]
fn negate_behavior() {
    assert_eq!(arithmetic::negate(Number::I8(5)), Ok(Number::I8(-5)));
    assert_eq!(
        arithmetic::negate(Number::I8(i8::MIN)),
        Err(Error::Overflow)
    );
    assert_eq!(
        arithmetic::negate(Number::U8(5)),
        Err(Error::UnsupportedTypes)
    );
    assert_eq!(
        arithmetic::negate(Number::U8(0)),
        Err(Error::UnsupportedTypes)
    );
    assert_eq!(arithmetic::negate(Number::F64(2.5)), Ok(Number::F64(-2.5)));
}

#[test]
fn number_size_of_check() {
    let size = std::mem::size_of::<Number>();
    assert!(size > 0);
}
